#![feature(test)]
#[cfg(test)]
extern crate test;
extern crate skiplist;
extern crate stable_skiplist;

use test::Bencher;

use std::collections::BTreeMap;
use std::collections::HashMap;
use skiplist::SkipMap;
use stable_skiplist::SkipMap as StableSkipMap;

pub const NUM :usize = 1_000;


#[bench]
fn btree(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut map = BTreeMap::new();

        for i in 0..NUM {
            let key = format!("KEY_{}", i);
            let val = format!("VAL_{}", i);

            map.insert(key, val);
        }
    });
}

#[bench]
fn skip_list(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut map = SkipMap::with_capacity(NUM);

        for i in 0..NUM {
            let key = format!("KEY_{}", i);
            let val = format!("VAL_{}", i);

            map.insert(key, val);
        }
    });
}

#[bench]
fn stable_skip_list(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut map = StableSkipMap::with_capacity(NUM);

        for i in 0..NUM {
            let key = format!("KEY_{}", i);
            let val = format!("VAL_{}", i);

            map.insert(key, val);
        }
    });
}

#[bench]
fn hash(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut map = HashMap::with_capacity(NUM);

        for i in 0..NUM {
            let key = format!("KEY_{}", i);
            let val = format!("VAL_{}", i);

            map.insert(key, val);
        }

        map.values().collect::<Vec<_>>().sort();
    });
}
