use crate as tinymap;
use tinymap::*;

#[test]
fn drop() {
    struct Foo(*mut i32);
    impl Foo {
        fn new(val: i32) -> Self {
            Foo(Box::into_raw(Box::new(val)))
        }
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropping {}", unsafe { self.0.as_ref() }.unwrap());
            unsafe { std::ptr::drop_in_place(self.0) };
        }
    }
    let mut map = arraymap!(i32 => Foo; 10);
    map.insert(1, Foo::new(2));
    map.insert(3, Foo::new(4));
}
