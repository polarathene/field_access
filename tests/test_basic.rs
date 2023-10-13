use field_access::{AccessError, FieldAccess};

#[test]
fn it_works() {
    #[allow(dead_code)]
    #[derive(FieldAccess)]
    struct Foo {
        a: u8,
    }

    let mut foo = Foo { a: 1 };

    assert!(foo.get_field("a").is_ok());
    assert!(foo.get_field("b").is_err());

    assert_eq!(foo.field("a").get::<&str>(), Err(AccessError::TypeMismatch));
    assert_eq!(foo.field("b").get::<u8>(), Err(AccessError::NoSuchField));

    assert_eq!(foo.field("a").as_u8(), Ok(1));
    assert!(foo.field_mut("a").set(2u8).is_ok());
    assert_eq!(foo.a, 2);
}