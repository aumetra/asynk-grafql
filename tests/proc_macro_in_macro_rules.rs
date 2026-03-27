#![allow(dead_code)]

#[tokio::test]
pub async fn test_object() {
    macro_rules! test_data {
        ($test_name:ident) => {
            #[derive(Debug, Clone)]
            pub struct $test_name {
                value: i32,
            }

            #[asynk_grafql::Object]
            impl $test_name {
                async fn value(&self) -> i32 {
                    self.value
                }
            }
        };
    }

    test_data!(A);
}

#[tokio::test]
pub async fn test_subscription() {
    macro_rules! test_data {
        ($test_name:ident) => {
            #[derive(Debug, Clone)]
            pub struct $test_name {
                value: i32,
            }

            #[asynk_grafql::Subscription]
            impl $test_name {
                async fn value(&self) -> impl futures_util::stream::Stream<Item = i32> + 'static {
                    let value = self.value;
                    futures_util::stream::once(async move { value })
                }
            }
        };
    }

    test_data!(A);
}

#[tokio::test]
pub async fn test_scalar() {
    macro_rules! test_data {
        ($test_name:ident) => {
            #[derive(Debug, Clone)]
            pub struct $test_name(i64);

            #[asynk_grafql::Scalar]
            impl asynk_grafql::ScalarType for $test_name {
                fn parse(value: asynk_grafql::Value) -> asynk_grafql::InputValueResult<Self> {
                    match value {
                        asynk_grafql::Value::Number(n) if n.is_i64() => {
                            Ok($test_name(n.as_i64().unwrap()))
                        }
                        _ => Err(asynk_grafql::InputValueError::expected_type(value)),
                    }
                }

                fn to_value(&self) -> asynk_grafql::Value {
                    self.0.to_value()
                }
            }
        };
    }

    test_data!(A);
}

#[tokio::test]
pub async fn test_oneof_object_type() {
    macro_rules! test_data {
        ($test_name:ident, $type1:ty, $type2:ty) => {
            #[derive(asynk_grafql::OneofObject)]
            enum $test_name {
                Type1($type1),
                Type2($type2),
            }
        };
    }

    #[derive(asynk_grafql::InputObject)]
    struct A {
        a: i32,
    }

    #[derive(asynk_grafql::InputObject)]
    struct B {
        b: i32,
    }

    test_data!(C, A, B);
}
