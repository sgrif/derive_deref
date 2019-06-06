#[macro_use]
extern crate derive_deref;

use std::marker::PhantomData;

#[test]
fn derive_deref_tuple_struct() {
    #[derive(Deref)]
    struct StringWrapper(String);
    #[derive(Deref)]
    struct IntWrapper(i32);

    assert_eq!("foo", &*StringWrapper(String::from("foo")));
    assert_eq!("bar", &*StringWrapper(String::from("bar")));
    assert_eq!(&1, &*IntWrapper(1));
    assert_eq!(1, *IntWrapper(1));
    assert_eq!(&2, &*IntWrapper(2));
    assert_eq!(2, *IntWrapper(2));
}

#[test]
fn derive_deref_tuple_struct_with_phantom_data() {
    trait FooTrait<T> {}
    impl FooTrait<()> for String {}

    #[derive(Deref)]
    struct FooWrapper<F, T>(F, PhantomData<T>) where F: FooTrait<T>;

    assert_eq!("foo", &*FooWrapper(String::from("foo"), PhantomData));
}

#[test]
fn derive_deref_named_struct() {
    #[derive(Deref)]
    struct StringWrapper { s: String };
    #[derive(Deref)]
    struct IntWrapper { i: i32 };

    assert_eq!("foo", &*StringWrapper { s: String::from("foo") });
    assert_eq!("bar", &*StringWrapper { s: String::from("bar") });
    assert_eq!(&1, &*IntWrapper { i: 1 });
    assert_eq!(1, *IntWrapper { i: 1 });
    assert_eq!(&2, &*IntWrapper { i: 2 });
    assert_eq!(2, *IntWrapper { i: 2 });
}

#[test]
fn derive_deref_named_struct_with_phantom_data() {
    trait FooTrait<T> {}
    impl FooTrait<()> for String {}

    #[derive(Deref)]
    struct FooNamedWrapper<F, T> where F: FooTrait<T> { f: F, phantom: PhantomData<T> };

    assert_eq!("foo", &*FooNamedWrapper { f: String::from("foo"), phantom: PhantomData });
}

#[test]
fn derive_deref_reference_ty() {
    #[derive(Deref)]
    struct StringWrapper<'a>(&'a str);

    assert_eq!("foo", &*StringWrapper("foo"));
    assert_eq!("bar", &*StringWrapper("bar"));
}

#[test]
fn derive_deref_ref_mut_ty() {
    #[derive(Deref)]
    struct StringWrapper<'a>(&'a mut str);

    let mut foo = String::from("foo");
    let mut bar = String::from("bar");
    assert_eq!("foo", &*StringWrapper(&mut foo));
    assert_eq!("bar", &*StringWrapper(&mut bar));
}

#[test]
fn derive_deref_generics() {
    #[derive(Deref)]
    struct Foo<T>(T);

    assert_eq!("foo", *Foo("foo"));
    assert_eq!(&"foo", &*Foo("foo"));
    assert_eq!(1, *Foo(1));
    assert_eq!(&1, &*Foo(1));
}

#[test]
fn derive_deref_mut() {
    #[derive(Deref, DerefMut)]
    struct StringWrapper(String);
    #[derive(Deref, DerefMut)]
    struct IntWrapper(i32);

    let mut foo = String::from("foo");
    let mut bar = String::from("bar");
    assert_eq!(&mut foo, &mut *StringWrapper(String::from("foo")));
    assert_eq!(&mut bar, &mut *StringWrapper(String::from("bar")));
    assert_eq!(&1, &mut *IntWrapper(1));
    assert_eq!(&2, &mut *IntWrapper(2));
}

#[test]
fn derive_deref_mut_ref() {
    #[derive(Deref, DerefMut)]
    struct StringWrapper<'a>(&'a mut str);

    let mut foo = String::from("foo");
    let mut foo2 = foo.clone();
    let mut bar = String::from("bar");
    let mut bar2 = bar.clone();
    assert_eq!(&mut foo, &mut *StringWrapper(&mut foo2));
    assert_eq!(&mut bar, &mut *StringWrapper(&mut bar2));
}
