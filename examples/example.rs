#![feature(custom_derive, proc_macro)]

#[macro_use] extern crate derive_enum_from_inner;

#[derive(Debug, EnumFromInner)]
#[allow(dead_code)]
enum Foo {
    A(i32),
    B(String),
    C(i32, i32),
}

fn main() {
    println!("{:?}", Foo::from("wow".to_string()));
    println!("{:?}", Foo::from(123));
}
