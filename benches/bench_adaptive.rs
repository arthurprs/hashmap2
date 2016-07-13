// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(test)]
#![feature(test)]

extern crate hashmap2;
extern crate test;

use test::Bencher;

use hashmap2::HashMap;

#[bench]
fn new_drop(b : &mut Bencher) {
    b.iter(|| {
        let m : HashMap<i32, i32> = HashMap::new();
        assert_eq!(m.len(), 0);
        test::black_box(&m);
    })
}

#[bench]
fn new_insert_drop(b : &mut Bencher) {
    b.iter(|| {
        let mut m = HashMap::new();
        m.insert(0, 0);
        assert_eq!(m.len(), 1);
        test::black_box(&m);
    })
}

#[bench]
fn grow_by_insertion(b: &mut Bencher) {
    let mut m = HashMap::new();

    for i in 1..1001 {
        m.insert(i, i);
    }

    test::black_box(&m);

    let mut k = 1001;

    b.iter(|| {
        m.insert(k, k);
        k += 1;
    });
    test::black_box(&m);
}

#[bench]
fn seq_insert(b: &mut Bencher) {
    let mut m = HashMap::with_capacity(1000);
    b.iter(|| {
        m.clear();
        for i in 0..1000 {
            m.insert(i, i);
        }
        test::black_box(&m);
    });
}

#[bench]
fn rev_insert(b: &mut Bencher) {
    let mut m = HashMap::with_capacity(1000);
    b.iter(|| {
        m.clear();
        for i in 0..1000 {
            let i = 10000000000 - i;
            m.insert(i, i);
        }
        test::black_box(&m);
    });
}

#[bench]
fn rng_insert(b: &mut Bencher) {
    let mut m = HashMap::with_capacity(1000);
    b.iter(|| {
        m.clear();
        let mut x = 1u64;
        for i in 0..1000 {
            m.insert(x, x);
            x ^= x >> 12; // a
            x ^= x << 25; // b
            x ^= x >> 27; // c
            x *= 2685821657736338717;
        }
        test::black_box(&m);
    });
}

#[bench]
fn find_existing(b: &mut Bencher) {
    let mut m = HashMap::new();

    for i in 1..1001 {
        m.insert(i, i);
    }

    test::black_box(&m);

    b.iter(|| {
        for i in 1..1001 {
            test::black_box(m.contains_key(&i));
        }
    });
}

#[bench]
fn find_nonexisting(b: &mut Bencher) {
    let mut m = HashMap::new();

    for i in 1..1001 {
        m.insert(i, i);
    }

    test::black_box(&m);

    b.iter(|| {
        for i in 1001..2001 {
            test::black_box(m.contains_key(&i));
        }
    });
}

#[bench]
fn hashmap_as_queue(b: &mut Bencher) {
    let mut m = HashMap::new();

    for i in 1..1001 {
        m.insert(i, i);
    }

    test::black_box(&m);

    let mut k = 1;

    b.iter(|| {
        m.remove(&k);
        m.insert(k + 1000, k + 1000);
        k += 1;
    });
    test::black_box(&m);
}

#[bench]
fn get_remove_insert(b: &mut Bencher) {
    let mut m = HashMap::new();

    for i in 1..1001 {
        m.insert(i, i);
    }

    test::black_box(&m);

    let mut k = 1;

    b.iter(|| {
        m.get(&(k + 400));
        m.get(&(k + 2000));
        m.remove(&k);
        m.insert(k + 1000, k + 1000);
        k += 1;
    });
    test::black_box(&m);
}
