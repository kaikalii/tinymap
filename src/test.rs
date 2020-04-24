use crate as tinymap;
use tinymap::*;

fn percent_change(a: f64, b: f64) -> f64 {
    (b - a) / a.abs() * 100.0
}

const M: usize = 10000;

#[test]
fn faster_than_hash_map() {
    use std::collections::HashMap;

    use eggtimer::measure;
    use rand::{thread_rng, Rng};

    const N: usize = 5;
    let mut sum = 0;
    let mut rng = thread_rng();
    let mut rand = || rng.gen_range(0, N);

    let mut hash_map = HashMap::<usize, usize>::new();
    let mut hash_map_insert = 0.0;
    let mut hash_map_get = 0.0;
    let mut hash_map_remove = 0.0;
    for _ in 0..M {
        hash_map_insert += measure(|| {
            for i in (0..N).map(|_| rand()) {
                hash_map.insert(i, i * 2);
            }
        });
        hash_map_get += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += *hash_map.get(&i).unwrap_or(&0);
            }
        });
        hash_map_remove += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += hash_map.remove(&i).unwrap_or(0);
            }
        });
        hash_map.clear();
    }

    let mut array_map = arraymap!(usize => usize; N);
    let mut array_map_insert = 0.0;
    let mut array_map_get = 0.0;
    let mut array_map_remove = 0.0;
    for _ in 0..M {
        array_map_insert += measure(|| {
            for i in (0..N).map(|_| rand()) {
                array_map.insert(i, i * 2);
            }
        });
        array_map_get += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += *array_map.get(&i).unwrap_or(&0);
            }
        });
        array_map_remove += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += array_map.remove(&i).unwrap_or(0);
            }
        });
        array_map.clear();
    }

    println!();
    println!(" hash_map_insert: {:.05} ms", hash_map_insert * 1000.0);
    println!("array_map_insert: {:.05} ms", array_map_insert * 1000.0);
    println!();
    println!("    hash_map_get: {:.05} ms", hash_map_get * 1000.0);
    println!("   array_map_get: {:.05} ms", array_map_get * 1000.0);
    println!();
    println!(" hash_map_remove: {:.05} ms", hash_map_remove * 1000.0);
    println!("array_map_remove: {:.05} ms", array_map_remove * 1000.0);
    println!();
    println!(
        "array_map_insert is {:.03} % faster",
        percent_change(1.0 / hash_map_insert, 1.0 / array_map_insert)
    );
    println!(
        "   array_map_get is {:.03} % faster",
        percent_change(1.0 / hash_map_get, 1.0 / array_map_get)
    );
    println!(
        "array_map_remove is {:.03} % faster",
        percent_change(1.0 / hash_map_remove, 1.0 / array_map_remove)
    );
    println!();

    assert!(array_map_insert < hash_map_insert);
    assert!(array_map_get < hash_map_get);
    assert!(array_map_remove < hash_map_remove);
}

#[test]
fn faster_than_btree_map() {
    use std::collections::BTreeMap;

    use eggtimer::measure;
    use rand::{thread_rng, Rng};

    const N: usize = 50;
    let mut sum = 0;
    let mut rng = thread_rng();
    let mut rand = || rng.gen_range(0, N);

    let mut btree_map = BTreeMap::<usize, usize>::new();
    let mut btree_map_insert = 0.0;
    let mut btree_map_get = 0.0;
    let mut btree_map_remove = 0.0;
    for _ in 0..M {
        btree_map_insert += measure(|| {
            for i in (0..N).map(|_| rand()) {
                btree_map.insert(i, i * 2);
            }
        });
        btree_map_get += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += *btree_map.get(&i).unwrap_or(&0);
            }
        });
        btree_map_remove += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += btree_map.remove(&i).unwrap_or(0);
            }
        });
        btree_map.clear();
    }

    let mut array_map = arraymap!(usize => usize; N);
    let mut array_map_insert = 0.0;
    let mut array_map_get = 0.0;
    let mut array_map_remove = 0.0;
    for _ in 0..M {
        array_map_insert += measure(|| {
            for i in (0..N).map(|_| rand()) {
                array_map.insert(i, i * 2);
            }
        });
        array_map_get += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += *array_map.get(&i).unwrap_or(&0);
            }
        });
        array_map_remove += measure(|| {
            for i in (0..N).map(|_| rand()) {
                sum += array_map.remove(&i).unwrap_or(0);
            }
        });
        array_map.clear();
    }

    println!();
    println!("btree_map_insert: {:.05} ms", btree_map_insert * 1000.0);
    println!("array_map_insert: {:.05} ms", array_map_insert * 1000.0);
    println!();
    println!("   btree_map_get: {:.05} ms", btree_map_get * 1000.0);
    println!("   array_map_get: {:.05} ms", array_map_get * 1000.0);
    println!();
    println!("btree_map_remove: {:.05} ms", btree_map_remove * 1000.0);
    println!("array_map_remove: {:.05} ms", array_map_remove * 1000.0);
    println!();
    println!(
        "array_map_insert is {:.03} % faster",
        percent_change(1.0 / btree_map_insert, 1.0 / array_map_insert)
    );
    println!(
        "   array_map_get is {:.03} % faster",
        percent_change(1.0 / btree_map_get, 1.0 / array_map_get)
    );
    println!(
        "array_map_remove is {:.03} % faster",
        percent_change(1.0 / btree_map_remove, 1.0 / array_map_remove)
    );
    println!();

    assert!(array_map_insert < btree_map_insert);
    assert!(array_map_remove < btree_map_remove);
}

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
