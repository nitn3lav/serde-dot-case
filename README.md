# serde-dot-case

Rename all enum fields to `dot.case`.

```rs
use serde::{Deserialize, Serialize};
use serde_dot_case::serde_dot_case;

#[serde_dot_case]
#[derive(Debug, Deserialize, Serialize)]
enum MyEnum {
    MyVariant,
}

#[test]
fn deserialize() {
    assert!(matches!(
        serde_json::from_str::<MyEnum>("\"my.variant\"").unwrap(),
        MyEnum::MyVariant
    ));
}

#[test]
fn serialize() {
    assert_eq!(
        "\"my.variant\"",
        serde_json::to_string(&MyEnum::MyVariant).unwrap(),
    );
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in serde-dot-case by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
