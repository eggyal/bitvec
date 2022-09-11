#![feature(test)]

extern crate test;

use bitvec::prelude::*;
use generic_array::typenum::{
	Shleft,
	Unsigned,
};
use test::Bencher;

type Len = Shleft<U1, U10>;

#[bench]
fn iter_proxy(bench: &mut Bencher) {
	let a = bits![1; Len];
	bench.iter(|| a.iter().all(|b| *b));
}

#[bench]
fn iter_ref(bench: &mut Bencher) {
	let a = bits![1; Len];
	bench.iter(|| a.iter().by_refs().all(|b| *b));
}

#[bench]
fn iter_val(bench: &mut Bencher) {
	let a = bits![1; Len];
	bench.iter(|| a.iter().by_vals().all(|b| b));
}

#[bench]
fn iter_bools(bench: &mut Bencher) {
	let a = [true; Len::USIZE];
	bench.iter(|| a.iter().copied().all(|b| b));
}
