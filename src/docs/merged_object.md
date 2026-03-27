Define a merged object with multiple object types.

*[See also the Book](https://asynk-grafql.github.io/asynk-grafql/en/merging_objects.html).*

# Macro attributes

| Attribute     | description                                                                                                                                                 | Type                                       | Optional |
|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------|----------|
| name          | Object name                                                                                                                                                 | string                                     | Y        |
| name_type     | If `true`, the object name will be specified from [`asynk_grafql::TypeName`](https://docs.rs/asynk-grafql/latest/asynk_grafql/trait.TypeName.html) trait | bool                                       | Y        |
| cache_control | Object cache control                                                                                                                                        | [`CacheControl`](struct.CacheControl.html) | Y        |
| extends       | Add fields to an entity that's defined in another service                                                                                                   | bool                                       | Y        |
| visible       | If `false`, it will not be displayed in introspection. *[See also the Book](https://asynk-grafql.github.io/asynk-grafql/en/visibility.html).*             | bool                                       | Y        |
| visible       | Call the specified function. If the return value is `false`, it will not be displayed in introspection.                                                     | string                                     | Y        |
| serial        | Resolve each field sequentially.                                                                                                                            | bool                                       | Y        |
| inaccessible  | Indicate that an object is not accessible from a supergraph when using Apollo Federation                                                                    | bool                                       | Y        |
| tag           | Arbitrary string metadata that will be propagated to the supergraph when using Apollo Federation. This attribute is repeatable                              | string                                     | Y        |
| directives    | Directives                                                                                                                                                  | expr                                       | Y        |

# Examples

```rust
use asynk_grafql::*;

#[derive(SimpleObject)]
 struct Object1 {
    a: i32,
 }

#[derive(SimpleObject)]
struct Object2 {
    b: i32,
}

#[derive(SimpleObject)]
struct Object3 {
    c: i32,
}

#[derive(MergedObject)]
struct MyObj(Object1, Object2, Object3);

let obj = MyObj(Object1 { a: 10 }, Object2 { b: 20 }, Object3 { c: 30 });
```

If you run into compilation errors like `error: queries overflow the depth limit!`, consider splitting the root object into smaller sets of objects merged with `MergedObject`:
```
use asynk_grafql::*;

#[derive(SimpleObject)]
struct Object1 {
    a: i32,
}

#[derive(SimpleObject)]
struct Object2 {
    b: i32,
}

#[derive(SimpleObject)]
struct Object3 {
    c: i32,
}

#[derive(MergedObject)]
struct MergeSet1(Object1, Object2);

#[derive(MergedObject)]
struct RootObject(MergeSet1, Object3);

let obj = RootObject(MergeSet1(Object1 { a: 10 }, Object2 { b: 20}), Object3 { c: 30});
 
```
