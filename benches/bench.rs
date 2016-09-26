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
        use hashmap2::unzip::HashMap as HashMap2Unzip;
        use hashmap2::zip::HashMap as HashMap2Zip;

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

        #[bench]
        fn insert_str_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            let ss = Vec::from_iter((0..c).map(|x| x.to_string()));
            b.iter(|| {
                let mut map = $hashmap::with_capacity(c);
                for key in &ss {
                    map.insert(&key[..], 0usize);
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
                    map.insert(x, x);
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
                    map.insert(x, x);
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
                    map.insert(x, x);
                }
                map
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

        #[bench]
        fn iter_values_100_000(b: &mut Bencher) {
            let c = 100_000usize;
            let mut map = $hashmap::with_capacity(c);
            for x in 0..c {
                map.insert(x, x);
            }
            b.iter(|| {
                map.values().sum::<usize>()
            });
        }

        #[bench]
        fn iter_keys_big_value_100_000(b: &mut Bencher) {
            let c = 100_000usize;
            let mut map = $hashmap::with_capacity(c);
            for x in 0..c {
                map.insert(x, [0u64; 10]);
            }
            b.iter(|| {
                map.keys().sum::<usize>()
            });
        }

        // #[bench]
        // fn iterate_1_000_000(b: &mut Bencher) {
        //     let c = 1_000_000usize;
        //     let mut map = $hashmap::with_capacity(c);
        //     for x in 0..c {
        //         map.insert(x, x);
        //     }
        //     b.iter(|| {
        //         map.iter().map(|(k, v)| *k + *v).sum::<usize>()
        //     });
        // }
        //
        // #[bench]
        // fn iter_keys_1_000_000(b: &mut Bencher) {
        //     let c = 1_000_000usize;
        //     let mut map = $hashmap::with_capacity(c);
        //     for x in 0..c {
        //         map.insert(x, x);
        //     }
        //     b.iter(|| {
        //         map.keys().sum::<usize>()
        //     });
        // }
        //
        // #[bench]
        // fn iter_values_1_000_000(b: &mut Bencher) {
        //     let c = 1_000_000usize;
        //     let mut map = $hashmap::with_capacity(c);
        //     for x in 0..c {
        //         map.insert(x, x);
        //     }
        //     b.iter(|| {
        //         map.values().sum::<usize>()
        //     });
        // }
        //
        // #[bench]
        // fn iter_keys_big_value_1_000_000(b: &mut Bencher) {
        //     let c = 1_000_000usize;
        //     let mut map = $hashmap::with_capacity(c);
        //     for x in 0..c {
        //         map.insert(x, [0u64; 10]);
        //     }
        //     b.iter(|| {
        //         map.keys().sum::<usize>()
        //     });
        // }

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
                for key in 0..5000 {
                    found += map.contains_key(&key) as u32;
                }
                found
            });
        }

        #[bench]
        fn lookup_10_000_noexist(b: &mut Bencher) {
            let c = 10_000usize;
            let mut map = HashMap::with_capacity(c);
            let keys = shuffled_keys(0..c);
            for &key in &keys {
                map.insert(key, 1);
            }
            b.iter(|| {
                let mut found = 0;
                for key in c..c+5000 {
                    found += map.contains_key(&key) as u32;
                }
                found
            });
        }


        // number of items to look up
        const LOOKUP_SAMPLE_SIZE: usize = 5000;


        lazy_static! {
            static ref HMAP_10K: $hashmap<usize, usize> = {
                let c = 10_000_usize;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..c);
                for &key in &keys {
                    map.insert(key, key);
                }
                map
            };

            static ref HMAP_100K: $hashmap<usize, usize> = {
                let c = 100_000_usize;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..c);
                for &key in &keys {
                    map.insert(key, key);
                }
                map
            };

            static ref HMAP_1M: $hashmap<usize, usize> = {
                let c = 1_000_000usize;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..c);
                for &key in &keys {
                    map.insert(key, key);
                }
                map
            };

            static ref HMAP_10K_BIG: $hashmap<usize, [u64; 10]> = {
                let c = 10_000_usize;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..c);
                for &key in &keys {
                    map.insert(key, [0u64;  10]);
                }
                map
            };
            static ref HMAP_100K_BIG: $hashmap<usize, [u64; 10]> = {
                let c = 100_000_usize;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..c);
                for &key in &keys {
                    map.insert(key, [0u64;  10]);
                }
                map
            };

            static ref HMAP_1M_BIG: $hashmap<usize, [u64; 10]> = {
                let c = 1_000_000_usize;
                let mut map = $hashmap::with_capacity(c as usize);
                let keys = shuffled_keys(0..c);
                for &key in &keys {
                    map.insert(key, [0u64;  10]);
                }
                map
            };
        }

        #[bench]
        fn lookup_10_000(b: &mut Bencher) {
            let map = &*HMAP_10K;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE {
                    found += *map.get(&key).unwrap();
                }
                found
            });
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

        #[bench]
        fn lookup_10_000_bigvalue(b: &mut Bencher) {
            let map = &*HMAP_10K_BIG;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE {
                    let f = map.get(&key).unwrap();
                    found += f[f.len() / 2];
                }
                found
            });
        }

        #[bench]
        fn lookup_100_000_bigvalue(b: &mut Bencher) {
            let map = &*HMAP_100K_BIG;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE {
                    let f = map.get(&key).unwrap();
                    found += f[f.len() / 2];
                }
                found
            });
        }

        #[bench]
        fn lookup_1_000_000_bigvalue(b: &mut Bencher) {
            let map = &*HMAP_1M_BIG;
            b.iter(|| {
                let mut found = 0;
                for key in 0..LOOKUP_SAMPLE_SIZE {
                    let f = map.get(&key).unwrap();
                    found += f[f.len() / 2];
                }
                found
            });
        }

        #[bench]
        fn lookup_1_000_000_bigvalue_unif(b: &mut Bencher) {
            let map = &*HMAP_1M_BIG;
            let mut keys = (0..map.len()).cycle();
            b.iter(|| {
                let mut found = 0;
                for key in keys.by_ref().take(LOOKUP_SAMPLE_SIZE) {
                    let f = map.get(&key).unwrap();
                    found += f[f.len() / 2];
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
        fn grow_fnv_10_000(b: &mut Bencher) {
            let c = 10_000usize;
            b.iter(|| {
                let mut map: $hashmap<_, _, FnvBuilder> = $hashmap::default();
                for x in 0..c {
                    map.insert(x, x);
                }
                map
            });
        }

        const MERGE: usize = 10_000usize;
        #[bench]
        fn merge_simple(b: &mut Bencher) {
            let first_map: $hashmap<usize, usize> = (0..MERGE).map(|i| (i, i)).collect();
            let second_map: $hashmap<usize, usize> = (MERGE..MERGE * 2).map(|i| (i, i)).collect();
            b.iter(|| {
                let mut merged = first_map.clone();
                merged.extend(second_map.iter().map(|(&k, &v)| (k, v)));
                merged
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
                merged.extend(v.drain(..));

                merged
            });
        }
    }};
}

// bench_mod!(hkvhkv, HashMap2Zip);
bench_mod!(hhkkvv, HashMap2Unzip);
bench_mod!(hhkvkv, HashMap2);
