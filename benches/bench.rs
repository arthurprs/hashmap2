#![feature(test)]
extern crate test;
extern crate rand;
extern crate fnv;
#[macro_use]
extern crate lazy_static;
extern crate hashmap2;
extern crate ordermap;

macro_rules! bench_mod {
    ($modname: ident, $hashmap: ident) => {
    mod $modname {
        use fnv::FnvHasher;
        use std::hash::BuildHasherDefault;
        type FnvBuilder = BuildHasherDefault<FnvHasher>;

        use test::Bencher;

        use hashmap2::HashMap as HashMap2;
        use hashmap2::adapt::HashMap as AdaptHashMap2;
        use hashmap2::adapt0::HashMap as AdaptHashMap0;
        use std::collections::HashMap;
        use std::iter::FromIterator;

        use rand::{XorShiftRng, Rng};

        fn weak_rng() -> XorShiftRng {
            XorShiftRng::new_unseeded()
        }

        fn shuffled_keys<I>(iter: I) -> Vec<I::Item>
            where I: IntoIterator
        {
            let mut v = Vec::from_iter(iter);
            let mut rng = weak_rng();
            rng.shuffle(&mut v);
            v
        }

        // number of items to look up
        const LOOKUP_SAMPLE_SIZE: usize = 5000;

        lazy_static! {
            static ref HMAP_100K: $hashmap<usize, usize> = {
                let c = 100_000_usize;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..map.capacity());
                // assert!(map.capacity() > 65_536);
                // assert!(map.capacity() < 131_072);
                for &key in &keys {
                    map.insert(key, key);
                }
                map
            };

            static ref HMAP_1M: $hashmap<usize, usize> = {
                let c = 1_000_000usize;
                let mut map = $hashmap::with_capacity(c as usize);
                // assert!(map.capacity() > 1_048_576);
                // assert!(map.capacity() < 2_097_152);
                let keys = shuffled_keys(0..map.capacity());
                for &key in &keys {
                    map.insert(key, key);
                }
                map
            };
        }


        #[bench]
        fn lookup_100_000(b: &mut Bencher) {
            let map = &*HMAP_100K;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE {
                    found += *map.get(&key).unwrap();
                }
                found
            });
        }

        #[bench]
        fn lookup_1_000_000(b: &mut Bencher) {
            let map = &*HMAP_1M;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE {
                    found += *map.get(&key).unwrap();
                }
                found
            });
        }
        //
        // #[bench]
        // fn lookup_100_000_unif(b: &mut Bencher) {
        //     let map = &*HMAP_100K;
        //     let mut keys = (0..map.len()).cycle();
        //     b.iter(|| {
        //         let mut found = 0;
        //         for key in keys.by_ref().take(LOOKUP_SAMPLE_SIZE) {
        //             found += *map.get(&key).unwrap();
        //         }
        //         found
        //     });
        // }
        //
        #[bench]
        fn lookup_1_000_000_unif(b: &mut Bencher) {
            let map = &*HMAP_1M;
            let mut keys = (0..map.len()).cycle();
            b.iter(|| {
                let mut found = 0;
                for key in keys.by_ref().take(LOOKUP_SAMPLE_SIZE) {
                    found += *map.get(&key).unwrap();
                }
                found
            });
        }
        // without preallocation
        #[bench]
        fn grow_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map = $hashmap::new();
                for x in 0..c {
                    map.insert(x, x);
                }
                map
            });
        }

        #[bench]
        fn clone_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            let mut map = $hashmap::new();
            for x in 0..c {
                map.insert(x, x);
            }
            b.iter(|| {
                map.clone()
            });
        }

        #[bench]
        fn grow_100_000(b: &mut Bencher) {
            let c = 100_000usize;
            b.iter(|| {
                let mut map = $hashmap::new();
                for x in 0..c {
                    map.insert(x, x);
                }
                map
            });
        }

        #[bench]
        fn grow_big_value_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map = $hashmap::new();
                for x in 0..c {
                    map.insert(x, [0u64; 10]);
                }
                map
            });
        }

        #[bench]
        fn grow_big_value_100_000(b: &mut Bencher) {
            let c = 100_000usize;
            b.iter(|| {
                let mut map = $hashmap::new();
                for x in 0..c {
                    map.insert(x, [0u64; 10]);
                }
                map
            });
        }


        #[bench]
        fn lru_sim(b: &mut Bencher) {
            let mut map = $hashmap::with_capacity(100_000usize);
            let c = map.capacity() * 11 / 10;
            for x in 0..c {
                map.insert(x, x);
            }
            b.iter(|| {
                for x in 0..c {
                    map.remove(&x);
                    map.insert(x, x);
                }
            });
        }

        #[bench]
        fn iterate_100_000(b: &mut Bencher) {
            let c = 100_000usize;
            let mut map = $hashmap::with_capacity(c);
            for x in 0..c {
                map.insert(x, x);
            }
            b.iter(|| {
                map.iter().map(|(k, v)| *k + *v).sum::<usize>()
            });
        }

        #[bench]
        fn iter_keys_100_000(b: &mut Bencher) {
            let c = 100_000usize;
            let mut map = $hashmap::with_capacity(c);
            for x in 0..c {
                map.insert(x, x);
            }
            b.iter(|| {
                map.keys().sum::<usize>()
            });
        }

        const MERGE: usize = 10_000usize;
        #[bench]
        fn merge_dos(b: &mut Bencher) {
            let first_map: $hashmap<usize, usize, FnvBuilder> = (0..MERGE).map(|i| (i, i)).collect();
            let second_map: $hashmap<usize, usize, FnvBuilder> = (MERGE..MERGE * 2).map(|i| (i, i)).collect();
            b.iter(|| {
                let mut merged = first_map.clone();
                for (&k, &v) in &second_map {
                    merged.insert(k, v);
                }
                ::test::black_box(merged);
            });
        }

        #[bench]
        fn merge_shuffle(b: &mut Bencher) {
            let first_map: $hashmap<usize, usize> = (0..MERGE).map(|i| (i, i)).collect();
            let second_map: $hashmap<usize, usize> = (MERGE..MERGE * 2).map(|i| (i, i)).collect();
            let mut v = Vec::new();
            let mut rng = weak_rng();
            b.iter(|| {
                let mut merged = first_map.clone();
                v.extend(second_map.iter().map(|(&k, &v)| (k, v)));
                rng.shuffle(&mut v);
                for (k, v) in v.drain(..) {
                    merged.insert(k, v);
                }

                ::test::black_box(merged);
            });
        }

        #[bench]
        fn insert_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                let c = map.capacity();
                for x in 0..c {
                    map.insert(x, x);
                }
                map
            });
        }

        #[bench]
        fn insert_string_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            let ss = Vec::from_iter((0..c).map(|x| x.to_string()));
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for key in &ss {
                    map.insert(key.clone(), 0usize);
                }
                map
            });
        }

        // #[bench]
        // fn insert_str_10_000(b: &mut Bencher) {
        //     let c = 10_000usize;
        //     let ss = Vec::from_iter((0..c).map(|x| x.to_string()));
        //     b.iter(|| {
        //         let mut map = $hashmap::with_capacity(c);
        //         for key in &ss {
        //             map.insert(&key[..], 0usize);
        //         }
        //         map
        //     });
        // }

        #[bench]
        fn insert_int_bigvalue_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            let value = [0u64; 10];
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                let c = map.capacity();
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
                let c = map.capacity();
                for x in 0..c {
                    map.insert(x, x);
                }
                map
            });
        }

        // #[bench]
        // fn insert_1_000_000(b: &mut Bencher) {
        //     let c = 1_000_000usize;
        //     b.iter(|| {
        //         let mut map = $hashmap::with_capacity(c);
        //         let c = map.capacity();
        //         for x in 0..c {
        //             map.insert(x, x);
        //         }
        //         map
        //     });
        // }
        //
        // #[bench]
        // fn insert_100(b: &mut Bencher) {
        //     let c = 100usize;
        //     b.iter(|| {
        //         let mut map = $hashmap::with_capacity(c);
        //         let c = map.capacity();
        //         for x in 0..c {
        //             map.insert(x, x);
        //         }
        //         map
        //     });
        // }

        #[bench]
        fn insert_1000(b: &mut Bencher) {
            let c = 1000usize;
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                let c = map.capacity();
                for x in 0..c {
                    map.insert(x, x);
                }
                map
            });
        }

    }};
}

bench_mod!(_11, HashMap2);
bench_mod!(pre, HashMap2);
bench_mod!(adp, AdaptHashMap2);
bench_mod!(adp0, AdaptHashMap0);
