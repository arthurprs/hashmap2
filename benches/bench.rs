#![feature(test)]
extern crate test;
extern crate rand;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
extern crate hashmap2;


macro_rules! bench_mod {
    ($modname: ident, $hashmap: ident) => {
    mod $modname {
        use fnv::FnvHasher;
        use std::hash::BuildHasherDefault;
        type FnvBuilder = BuildHasherDefault<FnvHasher>;

        use test::Bencher;

        use hashmap2::HashMap as HashMap2;

        use std::collections::HashMap;
        use std::iter::FromIterator;

        use rand::{weak_rng, Rng};

        #[bench]
        fn new(b: &mut Bencher) {
            b.iter(|| {
                $hashmap::<String, String>::new()
            });
        }

        #[bench]
        fn with_capacity_10e5(b: &mut Bencher) {
            b.iter(|| {
                $hashmap::<String, String>::with_capacity(10_000)
            });
        }

        #[bench]
        fn insert_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for x in 0..c {
                    map.insert(x, ());
                }
                map
            });
        }

        #[bench]
        fn insert_string_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for x in 0..c {
                    map.insert(x.to_string(), ());
                }
                map
            });
        }

        #[bench]
        fn insert_str_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            let ss = Vec::from_iter((0..c).map(|x| x.to_string()));
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for key in &ss {
                    map.insert(&key[..], ());
                }
                map
            });
        }

        #[bench]
        fn insert_int_bigvalue_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            let value = [0u64; 10];
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for i in 0..c {
                    map.insert(i, value);
                }
                map
            });
        }


        #[bench]
        fn insert_100_000(b: &mut Bencher) {
            let c = 100_000usize;
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for x in 0..c {
                    map.insert(x, ());
                }
                map
            });
        }

        #[bench]
        fn insert_100(b: &mut Bencher) {
            let c = 100usize;
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for x in 0..c {
                    map.insert(x, ());
                }
                map
            });
        }

        #[bench]
        fn insert_1000(b: &mut Bencher) {
            let c = 1000usize;
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for x in 0..c {
                    map.insert(x, ());
                }
                map
            });
        }

        #[bench]
        fn iterate_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            let mut map = $hashmap::with_capacity(c);
            let len = c - c/10;
            for x in 0..len {
                map.insert(x, ());
            }
            assert_eq!(map.len(), len);
            b.iter(|| {
                map.keys().sum::<usize>()
            });
        }

        fn shuffled_keys<I>(iter: I) -> Vec<I::Item>
            where I: IntoIterator
        {
            let mut v = Vec::from_iter(iter);
            let mut rng = weak_rng();
            rng.shuffle(&mut v);
            v
        }

        #[bench]
        fn lookup_10_000_exist(b: &mut Bencher) {
            let c = 10_000usize;
            let mut map = $hashmap::with_capacity(c);
            let keys = shuffled_keys(0..c);
            for &key in &keys {
                map.insert(key, 1);
            }
            b.iter(|| {
                let mut found = 0;
                for key in 5000..c {
                    found += map.get(&key).is_some() as i32;
                }
                found
            });
        }

        #[bench]
        fn lookup_hashmap_10_000_noexist(b: &mut Bencher) {
            let c = 10_000usize;
            let mut map = HashMap::with_capacity(c);
            let keys = shuffled_keys(0..c);
            for &key in &keys {
                map.insert(key, 1);
            }
            b.iter(|| {
                let mut found = 0;
                for key in c..15000 {
                    found += map.get(&key).is_some() as i32;
                }
                found
            });
        }


        // number of items to look up
        const LOOKUP_MAP_SIZE: usize = 100_000_usize;
        const LOOKUP_SAMPLE_SIZE: usize = 5000;


        lazy_static! {
            static ref HMAP_100K: $hashmap<usize, usize> = {
                let c = LOOKUP_MAP_SIZE;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..c);
                for &key in &keys {
                    map.insert(key, key);
                }
                map
            };
        }

        #[bench]
        fn lookup_100_000_multi(b: &mut Bencher) {
            let map = &*HMAP_100K;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE {
                    found += map.get(&key).is_some() as u32;
                }
                found
            });
        }

        #[bench]
        fn lookup_100_000_multi_10p(b: &mut Bencher) {
            let map = &*HMAP_100K;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE / 10 {
                    found += map.get(&key).is_some() as u32;
                }
                found
            });
        }


        #[bench]
        fn lookup_100_000_single(b: &mut Bencher) {
            let map = &*HMAP_100K;
            let mut iter = (0..LOOKUP_MAP_SIZE + LOOKUP_SAMPLE_SIZE).cycle();
            b.iter(|| {
                let key = iter.next().unwrap();
                map.get(&key).is_some()
            });
        }

        #[bench]
        fn lookup_100_000_single_10p(b: &mut Bencher) {
            let map = &*HMAP_100K;
            let mut iter = (0..LOOKUP_MAP_SIZE / 10).cycle();
            b.iter(|| {
                let key = iter.next().unwrap();
                map.get(&key).is_some()
            });
        }

        // without preallocation
        #[bench]
        fn grow_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map = $hashmap::new();
                for x in 0..c {
                    map.insert(x, ());
                }
                map
            });
        }

        #[bench]
        fn grow_fnv_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map: $hashmap<_, _, FnvBuilder> = $hashmap::default();
                for x in 0..c {
                    map.insert(x, ());
                }
                map
            });
        }

        const MERGE: usize = 10_000usize;
        #[bench]
        fn merge_simple(b: &mut Bencher) {
            let first_map: $hashmap<usize, _> = (0..MERGE).map(|i| (i, ())).collect();
            let second_map: $hashmap<usize, _> = (MERGE..MERGE * 2).map(|i| (i, ())).collect();
            b.iter(|| {
                let mut merged = first_map.clone();
                merged.extend(second_map.iter().map(|(&k, &v)| (k, v)));
                merged
            });
        }

        #[bench]
        fn merge_shuffle(b: &mut Bencher) {
            let first_map: $hashmap<usize, _> = (0..MERGE).map(|i| (i, ())).collect();
            let second_map: $hashmap<usize, _> = (MERGE..MERGE * 2).map(|i| (i, ())).collect();
            let mut v = Vec::new();
            let mut rng = weak_rng();
            b.iter(|| {
                let mut merged = first_map.clone();
                v.extend(second_map.iter().map(|(&k, &v)| (k, v)));
                rng.shuffle(&mut v);
                merged.extend(v.drain(..));

                merged
            });
        }
    }};
}

bench_mod!(hashmap, HashMap);
bench_mod!(hhhkvjv, HashMap2);
