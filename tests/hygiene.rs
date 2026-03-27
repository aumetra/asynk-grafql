#![allow(dead_code, non_camel_case_types, unused_macros)]
#![no_implicit_prelude]

// TODO: remove this: https://github.com/dtolnay/async-trait/issues/132
use ::asynk_grafql::{self, InputValueResult, ScalarType, Value};
use ::serde::{Deserialize, Serialize};
// TODO: remove this: https://github.com/nvzqz/static-assertions-rs/issues/37
use ::std::marker::Sized;

struct MyObject;
#[asynk_grafql::Object]
impl MyObject {
    #[graphql(deprecation = "abc")]
    async fn value(&self) -> ::std::primitive::i32 {
        5
    }
    async fn other_value(&self) -> &::std::primitive::i16 {
        &5
    }
    /// Add one to the number.
    async fn add_one(
        &self,
        #[graphql(default = 0)] v: ::std::primitive::i32,
    ) -> ::std::primitive::i32 {
        v + 1
    }
}

#[derive(asynk_grafql::SimpleObject)]
struct MySimpleObject {
    /// Value.
    #[graphql(owned)]
    value: ::std::primitive::i32,
    other_value: ::std::primitive::i16,
    #[graphql(deprecation = "bar")]
    bar: ::std::string::String,
    #[graphql(skip)]
    skipped: ::std::any::TypeId,
}

struct MySubscription;
#[asynk_grafql::Subscription]
impl MySubscription {
    #[graphql(deprecation = "abc")]
    async fn values(&self) -> impl ::futures_util::stream::Stream<Item = ::core::primitive::i32> {
        ::futures_util::stream::iter(5..7)
    }
    /// Count up from the value.
    async fn count_up_from(
        &self,
        #[graphql(default = 0)] v: ::std::primitive::i32,
    ) -> impl ::futures_util::stream::Stream<Item = ::core::primitive::i32> {
        ::futures_util::stream::iter(v..v + 20)
    }
}

struct MyScalar;
#[asynk_grafql::Scalar]
impl ScalarType for MyScalar {
    fn parse(_value: Value) -> InputValueResult<Self> {
        ::std::result::Result::Ok(Self)
    }
    fn to_value(&self) -> Value {
        Value::String(::std::borrow::ToOwned::to_owned("Hello world!"))
    }
}

#[derive(Serialize, Deserialize)]
struct MyScalar2(::std::primitive::i32);
::asynk_grafql::scalar!(MyScalar2);

#[derive(Clone, Copy, PartialEq, Eq, asynk_grafql::Enum)]
enum MyEnum {
    /// Foo.
    Foo,
    Bar,
}

#[derive(asynk_grafql::InputObject)]
struct MyInputObject {
    /// Foo.
    foo: ::std::primitive::i32,
    #[graphql(default)]
    bar: ::std::string::String,
}

#[derive(asynk_grafql::Interface)]
#[graphql(
    field(name = "value", ty = "::std::primitive::i32"),
    field(name = "other_value", ty = "&::std::primitive::i16")
)]
enum MyInterface {
    First(MyObject),
    Second(MySimpleObject),
}

#[derive(asynk_grafql::Union)]
enum MyUnion {
    First(MyObject),
    Second(MySimpleObject),
}

#[derive(asynk_grafql::MergedObject)]
struct MyMergedObject(MyObject, MySimpleObject);

#[derive(asynk_grafql::MergedSubscription)]
struct MyMergedSubscription(MySubscription);
