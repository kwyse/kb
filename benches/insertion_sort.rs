//! Insertion sort benchmarks

#![feature(test)]

extern crate kb;
extern crate rand;
extern crate test;

use kb::sort::insertion::*;

use rand::Rng;
use test::Bencher;

#[bench]
fn bench_clrs(b: &mut Bencher) {
    let mut arr = gen();
    b.iter(|| clrs(&mut arr));
}

#[bench]
fn bench_rev(b: &mut Bencher) {
    let mut arr = gen();
    b.iter(|| rev(&mut arr));
}

fn gen() -> Vec<u8> {
    let mut rng = ::rand::thread_rng();
    let mut vec = Vec::with_capacity(1_000);
    for _ in 0..1_000 {
        vec.push(rng.gen());
    }

    vec
}
