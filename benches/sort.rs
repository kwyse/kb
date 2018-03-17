//! Insertion sort benchmarks

#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate kb;
extern crate rand;
extern crate test;

use kb::sort::insertion::*;
use kb::sort::selection::*;

use rand::Rng;
use test::Bencher;

lazy_static! {
    static ref INPUT: Vec<u8> = gen_u8_vec(1_000);
}

#[bench]
fn insertion_sort_clrs_1000_u8(b: &mut Bencher) {
    let mut arr = INPUT.clone();
    b.iter(|| clrs(&mut arr));
}

#[bench]
fn insertion_sort_shepmaster_1000_u8(b: &mut Bencher) {
    let mut arr = INPUT.clone();
    b.iter(|| shepmaster(&mut arr));
}

#[bench]
fn selection_sort_me_1000_u8(b: &mut Bencher) {
    let mut arr = INPUT.clone();
    b.iter(|| selection(&mut arr));
}

fn gen_u8_vec(n: usize) -> Vec<u8> {
    let mut rng = ::rand::thread_rng();
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rng.gen());
    }

    vec
}
