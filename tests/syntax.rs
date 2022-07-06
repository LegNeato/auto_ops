use auto_ops::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Foo(i32);

#[test]
fn regular() {
    impl_op!(-|a: Foo, b: Foo| -> Foo { Foo(a.0 - b.0) });

    assert_eq!(Foo(1) - Foo(2), Foo(1 - 2));
}

#[test]
fn attributes() {
    impl_op!(
        &,
        #[inline]
        |a: Foo, b: Foo| -> Foo { Foo(a.0 & b.0) }
    );

    assert_eq!(Foo(1) & Foo(2), Foo(1 & 2));
}
#[test]
fn underscore() {
    impl_op!(+|_: Foo, _: Foo| -> Foo {
        Foo(5)
    });

    assert_eq!(Foo(1) + Foo(2), Foo(5));
}

#[test]
fn mutable() {
    impl_op!(*|mut a: Foo, b: Foo| -> Foo {
        a.0 *= b.0;
        a
    });

    assert_eq!(Foo(4) * Foo(2), Foo(8));
}

#[test]
#[allow(unused_parens)]
fn parens() {
    // Parens actually have a purpose

    impl_op!(/|(Foo(a)): Foo, (Foo(b)): Foo| -> Foo {
        Foo (a / b)
    });

    assert_eq!(Foo(4) / Foo(2), Foo(2));
}
