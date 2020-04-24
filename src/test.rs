use crate as tinymap;
use tinymap::*;

#[test]
fn drop_map() {
    use std::sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    };
    struct Foo(Arc<AtomicU32>);

    impl Drop for Foo {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    let counter = Arc::new(AtomicU32::new(0));
    let get_counter = || Arc::clone(&counter);

    {
        let mut map = arraymap!(i32 => Foo; 10);
        map.insert(1, Foo(get_counter()));
        map.insert(2, Foo(get_counter()));
        map.insert(3, Foo(get_counter()));
        map.insert(4, Foo(get_counter()));
        map.insert(5, Foo(get_counter()));

        assert_eq!(0, counter.load(Ordering::Relaxed));

        map.remove(&1);
        map.remove(&2);

        assert_eq!(2, counter.load(Ordering::Relaxed));
    }

    assert_eq!(5, counter.load(Ordering::Relaxed));
}

#[test]
fn drop_set() {
    use std::{
        borrow::Borrow,
        sync::{
            atomic::{AtomicU32, Ordering},
            Arc,
        },
    };

    #[derive(Debug)]
    struct Foo(i32, Arc<AtomicU32>);

    impl PartialEq for Foo {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl Eq for Foo {}

    impl PartialOrd for Foo {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Foo {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl Borrow<i32> for Foo {
        fn borrow(&self) -> &i32 {
            &self.0
        }
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            self.1.fetch_add(1, Ordering::Relaxed);
        }
    }

    let counter = Arc::new(AtomicU32::new(0));
    let get_counter = || Arc::clone(&counter);

    {
        let mut set = arrayset!(Foo; 10);
        set.insert(Foo(1, get_counter()));
        set.insert(Foo(2, get_counter()));
        set.insert(Foo(3, get_counter()));
        set.insert(Foo(4, get_counter()));
        set.insert(Foo(5, get_counter()));

        assert_eq!(0, counter.load(Ordering::Relaxed));

        set.remove(&1);
        set.remove(&2);

        assert_eq!(2, counter.load(Ordering::Relaxed));
    }

    assert_eq!(5, counter.load(Ordering::Relaxed));
}
