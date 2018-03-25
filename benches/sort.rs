//! Insertion sort benchmarks

#![feature(test)]

#[macro_use]
extern crate lazy_static;

extern crate kb;
extern crate rand;
extern crate test;

use kb::sort::insertion::*;
use kb::sort::merge::*;
use kb::sort::selection::*;

use rand::Rng;
use test::Bencher;

lazy_static! {
    static ref INPUT: Vec<u8> = gen_u8_vec(1_000);
    static ref SORTED_1000: [u8; 1000] = gen_sorted_1000();

    static ref SORTED_1000_REV: Vec<u8> = gen_sorted_1000()
        .iter().rev().map(Clone::clone).collect();
}

#[bench]
fn insertion_sort_clrs_1000_u8(b: &mut Bencher) {
    let mut arr = INPUT.clone();
    b.iter(|| clrs(&mut arr));
}

#[bench]
fn insertion_sort_clrs_1000_u8_sorted(b: &mut Bencher) {
    let mut arr = SORTED_1000.clone();
    b.iter(|| clrs(&mut arr));
}

#[bench]
fn insertion_sort_clrs_1000_u8_sorted_reversed(b: &mut Bencher) {
    let mut arr = SORTED_1000_REV.clone();
    b.iter(|| clrs(&mut arr));
}

#[bench]
fn insertion_sort_shepmaster_1000_u8(b: &mut Bencher) {
    let mut arr = INPUT.clone();
    b.iter(|| shepmaster(&mut arr));
}

#[bench]
fn merge_sort_clrs_1000_u8(b: &mut Bencher) {
    let mut arr = INPUT.clone().iter().map(|n| *n as f64).collect::<Vec<_>>();
    b.iter(|| clrs_merge_sort(&mut arr, 0, 1000));
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

fn gen_sorted_1000() -> [u8; 1000] {
    let mut values = [0u8; 1000];
    for i in 0..1000 {
        values[i] = (i / 4) as u8;
    }

    values
}
