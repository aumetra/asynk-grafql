use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures_util::lock::Mutex;
use serde::{Serialize, Serializer, ser::SerializeMap};

use crate::{
    Response, ServerResult, Value,
    extensions::{
        Extension, ExtensionContext, ExtensionFactory, NextExecute, NextResolve, ResolveInfo,
    },
    value,
};

struct ResolveState {
    path: Vec<String>,
    field_name: String,
    parent_type: String,
    return_type: String,
    start_offset: u128,
    duration_ns: u128,
}

impl Serialize for ResolveState {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("path", &self.path)?;
        map.serialize_entry("fieldName", &self.field_name)?;
        map.serialize_entry("parentType", &self.parent_type)?;
        map.serialize_entry("returnType", &self.return_type)?;
        map.serialize_entry("startOffset", &self.start_offset)?;
        map.serialize_entry("duration", &self.duration_ns)?;
        map.end()
    }
}

fn system_time_to_rfc3339(t: SystemTime) -> String {
    let duration = t.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    // Format as RFC3339 UTC timestamp
    let (y, mo, d, h, mi, s) = seconds_to_datetime(secs);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
        y, mo, d, h, mi, s, nanos
    )
}

fn seconds_to_datetime(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    // Days since epoch
    let days = secs / 86400;
    let time = secs % 86400;
    let h = time / 3600;
    let mi = (time % 3600) / 60;
    let s = time % 60;

    // Gregorian calendar calculation
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };
    (y, mo, d, h, mi, s)
}

/// Apollo tracing extension for performance tracing
///
/// Apollo Tracing works by including data in the extensions field of the
/// GraphQL response, which is reserved by the GraphQL spec for extra
/// information that a server wants to return. That way, you have access to
/// performance traces alongside the data returned by your query. It's already
/// supported by `Apollo Engine`, and we're excited to see what other kinds of
/// integrations people can build on top of this format.
#[cfg_attr(docsrs, doc(cfg(feature = "apollo_tracing")))]
pub struct ApolloTracing;

impl ExtensionFactory for ApolloTracing {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(ApolloTracingExtension {
            inner: Mutex::new(Inner {
                start_time: SystemTime::now(),
                end_time: SystemTime::now(),
                resolves: Default::default(),
            }),
        })
    }
}

struct Inner {
    start_time: SystemTime,
    end_time: SystemTime,
    resolves: Vec<ResolveState>,
}

struct ApolloTracingExtension {
    inner: Mutex<Inner>,
}

#[async_trait::async_trait]
impl Extension for ApolloTracingExtension {
    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        self.inner.lock().await.start_time = SystemTime::now();
        let resp = next.run(ctx, operation_name).await;

        let mut inner = self.inner.lock().await;
        inner.end_time = SystemTime::now();
        inner
            .resolves
            .sort_by(|a, b| a.start_offset.cmp(&b.start_offset));

        let duration_ns = inner
            .end_time
            .duration_since(inner.start_time)
            .unwrap_or(Duration::ZERO)
            .as_nanos();

        resp.extension(
            "tracing",
            value!({
                "version": 1,
                "startTime": system_time_to_rfc3339(inner.start_time),
                "endTime": system_time_to_rfc3339(inner.end_time),
                "duration": duration_ns,
                "execution": {
                    "resolvers": inner.resolves
                }
            }),
        )
    }

    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {
        let path = info.path_node.to_string_vec();
        let field_name = info.path_node.field_name().to_string();
        let parent_type = info.parent_type.to_string();
        let return_type = info.return_type.to_string();
        let resolve_start = SystemTime::now();
        let start_offset = resolve_start
            .duration_since(self.inner.lock().await.start_time)
            .unwrap_or(Duration::ZERO)
            .as_nanos();

        let res = next.run(ctx, info).await;
        let duration_ns = resolve_start
            .elapsed()
            .unwrap_or(Duration::ZERO)
            .as_nanos();

        self.inner.lock().await.resolves.push(ResolveState {
            path,
            field_name,
            parent_type,
            return_type,
            start_offset,
            duration_ns,
        });
        res
    }
}
