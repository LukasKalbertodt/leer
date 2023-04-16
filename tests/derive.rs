//! Tests the custom derive for `Empty`.
//!
//! We use complete path everywhere and avoid imports to make sure when the
//! derive code falsely assumes a symbol is in scope.


#[test]
fn unit_like() {
    #[derive(leer::Empty)]
    struct UnitLike;

    let _ = <UnitLike as leer::Empty>::empty();
}


#[test]
fn unnamed() {
    #[derive(leer::Empty)]
    struct UnnamedEmpty();

    #[derive(leer::Empty)]
    struct UnnamedSingle(String);

    #[derive(leer::Empty)]
    struct UnnamedTwo(String, Vec<u32>);


    let _ = <UnnamedEmpty as leer::Empty>::empty();
    let _ = <UnnamedSingle as leer::Empty>::empty();
    let _ = <UnnamedTwo as leer::Empty>::empty();
}


#[test]
fn named() {
    #![allow(dead_code)]

    #[derive(leer::Empty)]
    struct NamedEmpty {}

    #[derive(leer::Empty)]
    struct NamedSingle {
        string: String,
    }

    #[derive(leer::Empty)]
    struct NamedTwo {
        string: String,
        vec: Vec<u32>,
    }


    let _ = <NamedEmpty as leer::Empty>::empty();
    let _ = <NamedSingle as leer::Empty>::empty();
    let _ = <NamedTwo as leer::Empty>::empty();
}
