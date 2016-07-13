// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::hash::{BuildHasher, BuildHasherDefault, SipHasher, Hasher};

use sip_hash_state::SipHashState;

#[derive(Clone)]
pub struct AdaptiveState {
    inner: Option<SipHashState>,
}

impl AdaptiveState {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn new_fast() -> Self {
        AdaptiveState { inner: None }
    }

    #[inline]
    pub fn switch_to_safe_hashing(&mut self) {
        self.inner = Some(SipHashState::new());
    }

    pub fn uses_safe_hashing(&self) -> bool {
        self.inner.is_some()
    }
}

// For correct creation of HashMap.
impl Default for AdaptiveState {
    fn default() -> Self {
        let mut this = AdaptiveState::new_fast();
        this.switch_to_safe_hashing();
        this
    }
}

impl BuildHasher for AdaptiveState {
    type Hasher = AdaptiveHasher;
    #[inline]
    fn build_hasher(&self) -> AdaptiveHasher {
        AdaptiveHasher {
            safe_hasher: self.inner.as_ref().map(|state| state.build_hasher()),
            hash: 0,
        }
    }
}

pub struct AdaptiveHasher {
    safe_hasher: Option<SipHasher>,
    hash: u64,
}

impl Hasher for AdaptiveHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write(msg);
        } else {
            panic!();
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_u8(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_u16(&mut self, i: u16) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_u16(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_u32(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_u64(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_usize(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_i8(&mut self, i: i8) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_i8(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_i16(&mut self, i: i16) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_i16(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_i32(&mut self, i: i32) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_i32(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_i64(&mut self, i: i64) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_i64(i);
        } else {
            self.hash ^= i as u64;
        }
    }
    #[inline]
    fn write_isize(&mut self, i: isize) {
        if let Some(ref mut hasher) = self.safe_hasher {
            hasher.write_isize(i);
        } else {
            self.hash ^= i as u64;
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        if let Some(ref hasher) = self.safe_hasher {
            // Use safe hashing.
            hasher.finish()
        } else {
            // Use fast hashing.
            self.hash
        }
    }
}
