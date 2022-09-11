//! Invocation tests of each supported constructor-macro syntax.

#![cfg(test)]

use core::{
	cell::Cell,
	sync::atomic::*,
};

use generic_array::GenericArray;
use radium::types::*;

use crate::prelude::*;

#[test]
fn compile_bitarr_typedef() {
	#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
	struct Slots {
		all: BitArr!(for U10, in u8, Msb0),
		typ: BitArr!(for U10, in u8),
		def: BitArr!(for U10),
	}

	static SLOTS: Slots = Slots {
		all: bitarr!(const u8, Msb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
		typ: bitarr!(const u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
		def: bitarr!(const           1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
	};

	let slots = Slots {
		all: bitarr!(u8, Msb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
		typ: bitarr!(u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
		def: bitarr!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
	};

	assert_eq!(SLOTS, slots);

	assert_eq!(slots.all.into_inner(), arr![u8; !0, 192]);
	assert_eq!(slots.typ.into_inner(), arr![u8; !0, 3]);
	let def: GenericArray<usize, U1> = slots.def.into_inner();
	assert_eq!(def[0].count_ones(), 10);
}

#[test]
fn constexpr_macros() {
	const A: BitArr!(for U20, in Cell<u8>, Lsb0) =
		bitarr!(const Cell<u8>, Lsb0; 1; U20);
	let a = A;
	assert_eq!(a.len(), 20);
	assert!(a.all());

	const B: BitArr!(for U20) = bitarr!(const 1; U20);
	let b = B;
	assert_eq!(b.len(), 20);
	assert!(b.all());

	const C: BitArr!(for U5, in Cell<u16>, Msb0) =
		bitarr!(const Cell<u16>, Msb0; 1, 0, 1, 1, 0);
	let c = C;
	assert_eq!(c[.. 5], bits![1, 0, 1, 1, 0]);

	const D: BitArr!(for U5, in u32, Lsb0) =
		bitarr!(const u32, Lsb0; 1, 0, 1, 1, 0);
	let d = D;
	assert_eq!(d[.. 5], bits![1, 0, 1, 1, 0]);

	let _: &'static mut BitSlice<Cell<u16>, Msb0> =
		unsafe { bits!(static mut Cell<u16>, Msb0; 1; U20) };
	let _: &'static mut BitSlice<u32, Lsb0> =
		unsafe { bits!(static mut u32, Lsb0; 1; U20) };
	let _: &'static mut BitSlice = unsafe { bits!(static mut 1; U20) };

	let _: &'static mut BitSlice<Cell<u16>, Msb0> =
		unsafe { bits!(static mut Cell<u16>, Msb0; 1, 0, 1, 1, 0) };
	let _: &'static mut BitSlice<Cell<u32>, Msb0> =
		unsafe { bits!(static mut Cell<u32>, Msb0; 1, 0, 1, 1, 0) };
	let _: &'static mut BitSlice = unsafe { bits!(static mut 1, 0, 1, 1, 0) };

	let _: &'static BitSlice<Cell<u16>, Msb0> =
		bits!(static Cell<u16>, Msb0; 1; U20);
	let _: &'static BitSlice<u32, Lsb0> = bits!(static u32, Lsb0; 1, 0, 1, 1, 0);
	let _: &'static BitSlice = bits!(static 1; U20);

	let _: &'static BitSlice<Cell<u16>, Msb0> =
		bits!(static Cell<u16>, Msb0; 1, 0, 1, 1, 0);
	let _: &'static BitSlice<u32, Msb0> = bits!(static u32, Msb0; 1, 0, 1, 1, 0);
	let _: &'static BitSlice = bits!(static 1, 0, 1, 1, 0);
}

#[test]
fn compile_bitarr() {
	let uint: BitArray<u8, Lsb0, U4> = bitarr![u8, Lsb0; 1, 0, 1, 0];
	assert_eq!(uint.into_inner(), arr![u8; 5]);
	let cell: BitArray<Cell<u8>, Lsb0, U4> = bitarr![Cell<u8>, Lsb0; 1, 0, 1, 0];
	assert_eq!(cell.into_inner()[0].get(), 5u8);

	let uint: BitArray<u16, Msb0, U32> = bitarr![u16, Msb0;
		0, 1, 0, 1, 0, 1, 0, 1,
		0, 1, 1, 0, 1, 0, 0, 1,
		0, 1, 1, 0, 1, 1, 1, 0,
		0, 1, 1, 1, 0, 1, 0, 0,
	];
	assert_eq!(uint.into_inner(), arr![u16; 0x5569, 0x6E74]);
	let cell: BitArray<Cell<u16>, Msb0, U32> = bitarr![Cell<u16>, Msb0;
		0, 1, 0, 1, 0, 1, 0, 1,
		0, 1, 1, 0, 1, 0, 0, 1,
		0, 1, 1, 0, 1, 1, 1, 0,
		0, 1, 1, 1, 0, 1, 0, 0,
	];
	let cells = cell.into_inner();
	assert_eq!(cells[0].get(), 0x5569);
	assert_eq!(cells[1].get(), 0x6E74);

	let uint: BitArray<u32, Lsb0, U4> =
		bitarr![u32, crate::order::Lsb0; 1, 0, 1, 1];
	assert_eq!(uint.into_inner(), arr![u32; 13]);
	let cell: BitArray<Cell<u32>, Lsb0, U4> =
		bitarr![Cell<u32>, crate::order::Lsb0; 1, 0, 1, 1];
	assert_eq!(cell.into_inner()[0].get(), 13u32);

	#[cfg(target_pointer_width = "64")]
	{
		let uint: BitArray<u64, LocalBits, U70> =
			bitarr![u64, LocalBits; 1; U70];
		assert_eq!(uint.into_inner(), arr![u64; !0, !0]);

		let cell: BitArray<Cell<u64>, LocalBits, U70> =
			bitarr![Cell<u64>, LocalBits; 1; U70];
		assert_eq!(cell.clone().into_inner()[0].get(), !0u64);
		assert_eq!(cell.into_inner()[1].get(), !0u64);
	}

	let uint: BitArray<usize, Lsb0, U3> = bitarr![1, 0, 1];
	assert_eq!(uint.into_inner(), arr![usize; 5]);
	let uint: BitArray<usize, Lsb0, U30> = bitarr![1;U30];
	assert_eq!(uint.into_inner(), arr![usize; !0]);
}

#[test]
#[allow(clippy::many_single_char_names)]
fn compile_bits() {
	let a: &mut BitSlice<Cell<u8>, Lsb0> = bits![mut Cell<u8>, Lsb0; 1, 0, 1];
	let b: &mut BitSlice<u8, Lsb0> = bits![mut u8, Lsb0; 1, 0, 1];
	let c: &mut BitSlice<Cell<u8>, Msb0> =
		bits![mut Cell<u8>, crate::order::Msb0; 1, 0, 1];
	let d: &mut BitSlice<u8, Msb0> = bits![mut u8, crate::order::Msb0; 1, 0, 1];
	assert_eq!(a, c);
	assert_eq!(b, d);

	let e: &mut BitSlice<Cell<u8>, Lsb0> = bits![mut Cell<u8>, Lsb0; 1; U100];
	let f: &mut BitSlice<u8, Lsb0> = bits![mut u8, Lsb0; 1; U100];
	let g: &mut BitSlice<Cell<u8>, Msb0> =
		bits![mut Cell<u8>, crate::order::Msb0; 1; U100];
	let h: &mut BitSlice<u8, Msb0> = bits![mut u8, crate::order::Msb0; 1; U100];
	assert_eq!(e, g);
	assert_eq!(f, h);
	assert!(h.domain().take(12).all(|e| e == !0));
	assert_eq!(h.domain().next_back().unwrap(), 0b1111_0000);
	assert_eq!(h.domain().len(), 13);

	let i: &mut BitSlice<usize, Lsb0> = bits![mut 1, 0, 1];
	let j: &mut BitSlice<usize, Lsb0> = bits![mut 1; U3];
	j.set(1, false);
	assert_eq!(i, j);

	let _: &BitSlice<Cell<u8>, Lsb0> = bits![Cell<u8>, Lsb0; 1, 0, 1];
	let _: &BitSlice<u8, Lsb0> = bits![u8, Lsb0; 1, 0, 1];
	let _: &BitSlice<Cell<u8>, Msb0> =
		bits![Cell<u8>, crate::order::Msb0; 1, 0, 1];
	let _: &BitSlice<u8, Msb0> = bits![u8, crate::order::Msb0; 1, 0, 1];

	let _: &BitSlice<Cell<u8>, Lsb0> = bits![Cell<u8>, Lsb0; 1; U100];
	let _: &BitSlice<u8, Lsb0> = bits![u8, Lsb0; 1; U100];
	let _: &BitSlice<Cell<u8>, Msb0> =
		bits![Cell<u8>, crate::order::Msb0; 1; U100];
	let _: &BitSlice<u8, Msb0> = bits![u8, crate::order::Msb0; 1; U100];

	let _: &BitSlice<usize, Lsb0> = bits![1, 0, 1];
	let _: &BitSlice<usize, Lsb0> = bits![1; U100];

	let _: &BitSlice<Cell<u16>, Lsb0> = bits![Cell<u16>, Lsb0; 1, 0, 1];
	let _: &BitSlice<u16, Lsb0> = bits![u16, Lsb0; 1, 0, 1];
	let _: &BitSlice<Cell<u16>, Msb0> =
		bits![Cell<u16>, crate::order::Msb0; 1, 0, 1];
	let _: &BitSlice<u16, Msb0> = bits![u16, crate::order::Msb0; 1, 0, 1];

	let _: &BitSlice<Cell<u16>, Lsb0> = bits![Cell<u16>, Lsb0; 1; U100];
	let _: &BitSlice<u16, Lsb0> = bits![u16, Lsb0; 1; U100];
	let _: &BitSlice<Cell<u16>, Msb0> =
		bits![Cell<u16>, crate::order::Msb0; 1; U100];
	let _: &BitSlice<u16, Msb0> = bits![u16, crate::order::Msb0; 1; U100];

	let _: &BitSlice<Cell<u32>, Lsb0> = bits![Cell<u32>, Lsb0; 1, 0, 1];
	let _: &BitSlice<u32, Lsb0> = bits![u32, Lsb0; 1, 0, 1];
	let _: &BitSlice<Cell<u32>, Msb0> =
		bits![Cell<u32>, crate::order::Msb0; 1, 0, 1];
	let _: &BitSlice<u32, Msb0> = bits![u32, crate::order::Msb0; 1, 0, 1];

	let _: &BitSlice<Cell<u32>, Lsb0> = bits![Cell<u32>, Lsb0; 1; U100];
	let _: &BitSlice<u32, Lsb0> = bits![u32, Lsb0; 1; U100];
	let _: &BitSlice<Cell<u32>, Msb0> =
		bits![Cell<u32>, crate::order::Msb0; 1; U100];
	let _: &BitSlice<u32, Msb0> = bits![u32, crate::order::Msb0; 1; U100];

	let _: &BitSlice<Cell<usize>, Lsb0> = bits![Cell<usize>, Lsb0; 1, 0, 1];
	let _: &BitSlice<usize, Lsb0> = bits![usize, Lsb0; 1, 0, 1];
	let _: &BitSlice<Cell<usize>, Msb0> =
		bits![Cell<usize>, crate::order::Msb0; 1, 0, 1];
	let _: &BitSlice<usize, Msb0> = bits![usize, crate::order::Msb0; 1, 0, 1];

	let _: &BitSlice<Cell<usize>, Lsb0> = bits![Cell<usize>, Lsb0; 1; U100];
	let _: &BitSlice<usize, Lsb0> = bits![usize, Lsb0; 1; U100];
	let _: &BitSlice<Cell<usize>, Msb0> =
		bits![Cell<usize>, crate::order::Msb0; 1; U100];
	let _: &BitSlice<usize, Msb0> = bits![usize, crate::order::Msb0; 1; U100];

	#[cfg(target_pointer_width = "64")]
	{
		let _: &BitSlice<Cell<u64>, Lsb0> = bits![Cell<u64>, Lsb0; 1, 0, 1];
		let _: &BitSlice<u64, Lsb0> = bits![u64, Lsb0; 1, 0, 1];
		let _: &BitSlice<Cell<u64>, Msb0> =
			bits![Cell<u64>, crate::order::Msb0; 1, 0, 1];
		let _: &BitSlice<u64, Msb0> = bits![u64, crate::order::Msb0; 1, 0, 1];

		let _: &BitSlice<Cell<u64>, Lsb0> = bits![Cell<u64>, Lsb0; 1; U100];
		let _: &BitSlice<u64, Lsb0> = bits![u64, Lsb0; 1; U100];
		let _: &BitSlice<Cell<u64>, Msb0> =
			bits![Cell<u64>, crate::order::Msb0; 1; U100];
		let _: &BitSlice<u64, Msb0> = bits![u64, crate::order::Msb0; 1; U100];
	}

	radium::if_atomic! {
		if atomic(8) {
			let _: &BitSlice<AtomicU8, LocalBits> = bits![AtomicU8, LocalBits; 0, 1];
			let _: &BitSlice<AtomicU8, Lsb0> = bits![AtomicU8, Lsb0; 0, 1];
			let _: &BitSlice<AtomicU8, Msb0> = bits![AtomicU8, Msb0; 0, 1];
			let _: &BitSlice<RadiumU8, LocalBits> = bits![RadiumU8, LocalBits; 1; U100];
			let _: &BitSlice<RadiumU8, Lsb0> = bits![RadiumU8, Lsb0; 1; U100];
			let _: &BitSlice<RadiumU8, Msb0> = bits![RadiumU8, Msb0; 1; U100];
		}
		if atomic(16) {
			let _: &BitSlice<AtomicU16, LocalBits> = bits![AtomicU16, LocalBits; 0, 1];
			let _: &BitSlice<AtomicU16, Lsb0> = bits![AtomicU16, Lsb0; 0, 1];
			let _: &BitSlice<AtomicU16, Msb0> = bits![AtomicU16, Msb0; 0, 1];
			let _: &BitSlice<RadiumU16, LocalBits> = bits![RadiumU16, LocalBits; 1; U100];
			let _: &BitSlice<RadiumU16, Lsb0> = bits![RadiumU16, Lsb0; 1; U100];
			let _: &BitSlice<RadiumU16, Msb0> = bits![RadiumU16, Msb0; 1; U100];
		}
		if atomic(32) {
			let _: &BitSlice<AtomicU32, LocalBits> = bits![AtomicU32, LocalBits; 0, 1];
			let _: &BitSlice<AtomicU32, Lsb0> = bits![AtomicU32, Lsb0; 0, 1];
			let _: &BitSlice<AtomicU32, Msb0> = bits![AtomicU32, Msb0; 0, 1];
			let _: &BitSlice<RadiumU32, LocalBits> = bits![RadiumU32, LocalBits; 1; U100];
			let _: &BitSlice<RadiumU32, Lsb0> = bits![RadiumU32, Lsb0; 1; U100];
			let _: &BitSlice<RadiumU32, Msb0> = bits![RadiumU32, Msb0; 1; U100];
		}
		if atomic(size) {
			let _: &BitSlice<AtomicUsize, LocalBits> = bits![AtomicUsize, LocalBits; 0, 1];
			let _: &BitSlice<AtomicUsize, Lsb0> = bits![AtomicUsize, Lsb0; 0, 1];
			let _: &BitSlice<AtomicUsize, Msb0> = bits![AtomicUsize, Msb0; 0, 1];
			let _: &BitSlice<RadiumUsize, LocalBits> = bits![RadiumUsize, LocalBits; 1; U100];
			let _: &BitSlice<RadiumUsize, Lsb0> = bits![RadiumUsize, Lsb0; 1; U100];
			let _: &BitSlice<RadiumUsize, Msb0> = bits![RadiumUsize, Msb0; 1; U100];
		}
	}
	#[cfg(target_pointer_width = "64")]
	radium::if_atomic! {
		if atomic(64) {
			let _: &BitSlice<AtomicU64, LocalBits> = bits![AtomicU64, LocalBits; 0, 1];
			let _: &BitSlice<AtomicU64, Lsb0> = bits![AtomicU64, Lsb0; 0, 1];
			let _: &BitSlice<AtomicU64, Msb0> = bits![AtomicU64, Msb0; 0, 1];
			let _: &BitSlice<RadiumU64, LocalBits> = bits![RadiumU64, LocalBits; 1; U100];
			let _: &BitSlice<RadiumU64, Lsb0> = bits![RadiumU64, Lsb0; 1; U100];
			let _: &BitSlice<RadiumU64, Msb0> = bits![RadiumU64, Msb0; 1; U100];
		}
	}
}

#[test]
#[cfg(feature = "alloc")]
fn compile_bitvec() {
	let _: BitVec<Cell<u8>, Lsb0> = bitvec![Cell<u8>, Lsb0; 1, 0, 1];
	let _: BitVec<u8, Lsb0> = bitvec![u8, Lsb0; 1, 0, 1];
	let _: BitVec<Cell<u8>, Msb0> =
		bitvec![Cell<u8>, crate::order::Msb0; 1, 0, 1];
	let _: BitVec<u8, Msb0> = bitvec![u8, crate::order::Msb0; 1, 0, 1];

	let _: BitVec<Cell<u8>, Lsb0> = bitvec![Cell<u8>, Lsb0; 1; 100];
	let _: BitVec<u8, Lsb0> = bitvec![u8, Lsb0; 1; 100];
	let _: BitVec<Cell<u8>, Msb0> =
		bitvec![Cell<u8>, crate::order::Msb0; 1; 100];
	let _: BitVec<u8, Msb0> = bitvec![u8, crate::order::Msb0; 1; 100];

	let _: BitVec<usize, Lsb0> = bitvec![1, 0, 1];
	let _: BitVec<usize, Lsb0> = bitvec![1; 100];

	let _: BitVec<Cell<u16>, Lsb0> = bitvec![Cell<u16>, Lsb0; 1, 0, 1];
	let _: BitVec<u16, Lsb0> = bitvec![u16, Lsb0; 1, 0, 1];
	let _: BitVec<Cell<u16>, Msb0> =
		bitvec![Cell<u16>, crate::order::Msb0; 1, 0, 1];
	let _: BitVec<u16, Msb0> = bitvec![u16, crate::order::Msb0; 1, 0, 1];

	let _: BitVec<Cell<u16>, Lsb0> = bitvec![Cell<u16>, Lsb0; 1; 100];
	let _: BitVec<u16, Lsb0> = bitvec![u16, Lsb0; 1; 100];
	let _: BitVec<Cell<u16>, Msb0> =
		bitvec![Cell<u16>, crate::order::Msb0; 1; 100];
	let _: BitVec<u16, Msb0> = bitvec![u16, crate::order::Msb0; 1; 100];

	let _: BitVec<Cell<u32>, Lsb0> = bitvec![Cell<u32>, Lsb0; 1, 0, 1];
	let _: BitVec<u32, Lsb0> = bitvec![u32, Lsb0; 1, 0, 1];
	let _: BitVec<Cell<u32>, Msb0> =
		bitvec![Cell<u32>, crate::order::Msb0; 1, 0, 1];
	let _: BitVec<u32, Msb0> = bitvec![u32, crate::order::Msb0; 1, 0, 1];

	let _: BitVec<Cell<u32>, Lsb0> = bitvec![Cell<u32>, Lsb0; 1; 100];
	let _: BitVec<u32, Lsb0> = bitvec![u32, Lsb0; 1; 100];
	let _: BitVec<Cell<u32>, Msb0> =
		bitvec![Cell<u32>, crate::order::Msb0; 1; 100];
	let _: BitVec<u32, Msb0> = bitvec![u32, crate::order::Msb0; 1; 100];

	let _: BitVec<Cell<usize>, Lsb0> = bitvec![Cell<usize>, Lsb0; 1, 0, 1];
	let _: BitVec<usize, Lsb0> = bitvec![usize, Lsb0; 1, 0, 1];
	let _: BitVec<Cell<usize>, Msb0> =
		bitvec![Cell<usize>, crate::order::Msb0; 1, 0, 1];
	let _: BitVec<usize, Msb0> = bitvec![usize, crate::order::Msb0; 1, 0, 1];

	let _: BitVec<Cell<usize>, Lsb0> = bitvec![Cell<usize>, Lsb0; 1; 100];
	let _: BitVec<usize, Lsb0> = bitvec![usize, Lsb0; 1; 100];
	let _: BitVec<Cell<usize>, Msb0> =
		bitvec![Cell<usize>, crate::order::Msb0; 1; 100];
	let _: BitVec<usize, Msb0> = bitvec![usize, crate::order::Msb0; 1; 100];

	#[cfg(target_pointer_width = "64")]
	{
		let _: BitVec<Cell<u64>, Lsb0> = bitvec![Cell<u64>, Lsb0; 1, 0, 1];
		let _: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 1, 0, 1];
		let _: BitVec<Cell<u64>, Msb0> =
			bitvec![Cell<u64>, crate::order::Msb0; 1, 0, 1];
		let _: BitVec<u64, Msb0> = bitvec![u64, crate::order::Msb0; 1, 0, 1];

		let _: BitVec<Cell<u64>, Lsb0> = bitvec![Cell<u64>, Lsb0; 1; 100];
		let _: BitVec<u64, Lsb0> = bitvec![u64, Lsb0; 1; 100];
		let _: BitVec<Cell<u64>, Msb0> =
			bitvec![Cell<u64>, crate::order::Msb0; 1; 100];
		let _: BitVec<u64, Msb0> = bitvec![u64, crate::order::Msb0; 1; 100];
	}
	radium::if_atomic! {
		if atomic(8) {
			let _: BitVec<AtomicU8, LocalBits> =bitvec![AtomicU8, LocalBits; 0, 1];
			let _: BitVec<AtomicU8, Lsb0> =bitvec![AtomicU8, Lsb0; 0, 1];
			let _: BitVec<AtomicU8, Msb0> =bitvec![AtomicU8, Msb0; 0, 1];
			let _: BitVec<RadiumU8, LocalBits> =bitvec![RadiumU8, LocalBits; 1; 100];
			let _: BitVec<RadiumU8, Lsb0> =bitvec![RadiumU8, Lsb0; 1; 100];
			let _: BitVec<RadiumU8, Msb0> =bitvec![RadiumU8, Msb0; 1; 100];
		}
		if atomic(16) {
			let _: BitVec<AtomicU16, LocalBits> =bitvec![AtomicU16, LocalBits; 0, 1];
			let _: BitVec<AtomicU16, Lsb0> =bitvec![AtomicU16, Lsb0; 0, 1];
			let _: BitVec<AtomicU16, Msb0> =bitvec![AtomicU16, Msb0; 0, 1];
			let _: BitVec<RadiumU16, LocalBits> =bitvec![RadiumU16, LocalBits; 1; 100];
			let _: BitVec<RadiumU16, Lsb0> =bitvec![RadiumU16, Lsb0; 1; 100];
			let _: BitVec<RadiumU16, Msb0> =bitvec![RadiumU16, Msb0; 1; 100];
		}
		if atomic(32) {
			let _: BitVec<AtomicU32, LocalBits> =bitvec![AtomicU32, LocalBits; 0, 1];
			let _: BitVec<AtomicU32, Lsb0> =bitvec![AtomicU32, Lsb0; 0, 1];
			let _: BitVec<AtomicU32, Msb0> =bitvec![AtomicU32, Msb0; 0, 1];
			let _: BitVec<RadiumU32, LocalBits> =bitvec![RadiumU32, LocalBits; 1; 100];
			let _: BitVec<RadiumU32, Lsb0> =bitvec![RadiumU32, Lsb0; 1; 100];
			let _: BitVec<RadiumU32, Msb0> =bitvec![RadiumU32, Msb0; 1; 100];
		}
		if atomic(size) {
			let _: BitVec<AtomicUsize, LocalBits> =bitvec![AtomicUsize, LocalBits; 0, 1];
			let _: BitVec<AtomicUsize, Lsb0> =bitvec![AtomicUsize, Lsb0; 0, 1];
			let _: BitVec<AtomicUsize, Msb0> =bitvec![AtomicUsize, Msb0; 0, 1];
			let _: BitVec<RadiumUsize, LocalBits> =bitvec![RadiumUsize, LocalBits; 1; 100];
			let _: BitVec<RadiumUsize, Lsb0> =bitvec![RadiumUsize, Lsb0; 1; 100];
			let _: BitVec<RadiumUsize, Msb0> =bitvec![RadiumUsize, Msb0; 1; 100];
		}
	}
	#[cfg(target_pointer_width = "64")]
	radium::if_atomic! {
		if atomic(64) {
			let _: BitVec<AtomicU64, LocalBits> =bitvec![AtomicU64, LocalBits; 0, 1];
			let _: BitVec<AtomicU64, Lsb0> =bitvec![AtomicU64, Lsb0; 0, 1];
			let _: BitVec<AtomicU64, Msb0> =bitvec![AtomicU64, Msb0; 0, 1];
			let _: BitVec<RadiumU64, LocalBits> =bitvec![RadiumU64, LocalBits; 1; 100];
			let _: BitVec<RadiumU64, Lsb0> =bitvec![RadiumU64, Lsb0; 1; 100];
			let _: BitVec<RadiumU64, Msb0> =bitvec![RadiumU64, Msb0; 1; 100];
		}
	}
}

#[test]
#[cfg(feature = "alloc")]
fn compile_bitbox() {
	let _: BitBox<Cell<u8>, Lsb0> = bitbox![Cell<u8>, Lsb0; 1, 0, 1];
	let _: BitBox<u8, Lsb0> = bitbox![u8, Lsb0; 1, 0, 1];
	let _: BitBox<Cell<u8>, Msb0> =
		bitbox![Cell<u8>, crate::order::Msb0; 1, 0, 1];
	let _: BitBox<u8, Msb0> = bitbox![u8, crate::order::Msb0; 1, 0, 1];

	let _: BitBox<Cell<u8>, Lsb0> = bitbox![Cell<u8>, Lsb0; 1; 100];
	let _: BitBox<u8, Lsb0> = bitbox![u8, Lsb0; 1; 100];
	let _: BitBox<Cell<u8>, Msb0> =
		bitbox![Cell<u8>, crate::order::Msb0; 1; 100];
	let _: BitBox<u8, Msb0> = bitbox![u8, crate::order::Msb0; 1; 100];

	let _: BitBox<usize, Lsb0> = bitbox![1, 0, 1];
	let _: BitBox<usize, Lsb0> = bitbox![1; 100];

	let _: BitBox<Cell<u16>, Lsb0> = bitbox![Cell<u16>, Lsb0; 1, 0, 1];
	let _: BitBox<u16, Lsb0> = bitbox![u16, Lsb0; 1, 0, 1];
	let _: BitBox<Cell<u16>, Msb0> =
		bitbox![Cell<u16>, crate::order::Msb0; 1, 0, 1];
	let _: BitBox<u16, Msb0> = bitbox![u16, crate::order::Msb0; 1, 0, 1];

	let _: BitBox<Cell<u16>, Lsb0> = bitbox![Cell<u16>, Lsb0; 1; 100];
	let _: BitBox<u16, Lsb0> = bitbox![u16, Lsb0; 1; 100];
	let _: BitBox<Cell<u16>, Msb0> =
		bitbox![Cell<u16>, crate::order::Msb0; 1; 100];
	let _: BitBox<u16, Msb0> = bitbox![u16, crate::order::Msb0; 1; 100];

	let _: BitBox<Cell<u32>, Lsb0> = bitbox![Cell<u32>, Lsb0; 1, 0, 1];
	let _: BitBox<u32, Lsb0> = bitbox![u32, Lsb0; 1, 0, 1];
	let _: BitBox<Cell<u32>, Msb0> =
		bitbox![Cell<u32>, crate::order::Msb0; 1, 0, 1];
	let _: BitBox<u32, Msb0> = bitbox![u32, crate::order::Msb0; 1, 0, 1];

	let _: BitBox<Cell<u32>, Lsb0> = bitbox![Cell<u32>, Lsb0; 1; 100];
	let _: BitBox<u32, Lsb0> = bitbox![u32, Lsb0; 1; 100];
	let _: BitBox<Cell<u32>, Msb0> =
		bitbox![Cell<u32>, crate::order::Msb0; 1; 100];
	let _: BitBox<u32, Msb0> = bitbox![u32, crate::order::Msb0; 1; 100];

	let _: BitBox<Cell<usize>, Lsb0> = bitbox![Cell<usize>, Lsb0; 1, 0, 1];
	let _: BitBox<usize, Lsb0> = bitbox![usize, Lsb0; 1, 0, 1];
	let _: BitBox<Cell<usize>, Msb0> =
		bitbox![Cell<usize>, crate::order::Msb0; 1, 0, 1];
	let _: BitBox<usize, Msb0> = bitbox![usize, crate::order::Msb0; 1, 0, 1];

	let _: BitBox<Cell<usize>, Lsb0> = bitbox![Cell<usize>, Lsb0; 1; 100];
	let _: BitBox<usize, Lsb0> = bitbox![usize, Lsb0; 1; 100];
	let _: BitBox<Cell<usize>, Msb0> =
		bitbox![Cell<usize>, crate::order::Msb0; 1; 100];
	let _: BitBox<usize, Msb0> = bitbox![usize, crate::order::Msb0; 1; 100];

	#[cfg(target_pointer_width = "64")]
	{
		let _: BitBox<Cell<u64>, Lsb0> = bitbox![Cell<u64>, Lsb0; 1, 0, 1];
		let _: BitBox<u64, Lsb0> = bitbox![u64, Lsb0; 1, 0, 1];
		let _: BitBox<Cell<u64>, Msb0> =
			bitbox![Cell<u64>, crate::order::Msb0; 1, 0, 1];
		let _: BitBox<u64, Msb0> = bitbox![u64, crate::order::Msb0; 1, 0, 1];

		let _: BitBox<Cell<u64>, Lsb0> = bitbox![Cell<u64>, Lsb0; 1; 100];
		let _: BitBox<u64, Lsb0> = bitbox![u64, Lsb0; 1; 100];
		let _: BitBox<Cell<u64>, Msb0> =
			bitbox![Cell<u64>, crate::order::Msb0; 1; 100];
		let _: BitBox<u64, Msb0> = bitbox![u64, crate::order::Msb0; 1; 100];
	}
	radium::if_atomic! {
		if atomic(8) {
			let _: BitBox<AtomicU8, LocalBits> =bitbox![AtomicU8, LocalBits; 0, 1];
			let _: BitBox<AtomicU8, Lsb0> =bitbox![AtomicU8, Lsb0; 0, 1];
			let _: BitBox<AtomicU8, Msb0> =bitbox![AtomicU8, Msb0; 0, 1];
			let _: BitBox<RadiumU8, LocalBits> =bitbox![RadiumU8, LocalBits; 1; 100];
			let _: BitBox<RadiumU8, Lsb0> =bitbox![RadiumU8, Lsb0; 1; 100];
			let _: BitBox<RadiumU8, Msb0> =bitbox![RadiumU8, Msb0; 1; 100];
		}
		if atomic(16) {
			let _: BitBox<AtomicU16, LocalBits> =bitbox![AtomicU16, LocalBits; 0, 1];
			let _: BitBox<AtomicU16, Lsb0> =bitbox![AtomicU16, Lsb0; 0, 1];
			let _: BitBox<AtomicU16, Msb0> =bitbox![AtomicU16, Msb0; 0, 1];
			let _: BitBox<RadiumU16, LocalBits> =bitbox![RadiumU16, LocalBits; 1; 100];
			let _: BitBox<RadiumU16, Lsb0> =bitbox![RadiumU16, Lsb0; 1; 100];
			let _: BitBox<RadiumU16, Msb0> =bitbox![RadiumU16, Msb0; 1; 100];
		}
		if atomic(32) {
			let _: BitBox<AtomicU32, LocalBits> =bitbox![AtomicU32, LocalBits; 0, 1];
			let _: BitBox<AtomicU32, Lsb0> =bitbox![AtomicU32, Lsb0; 0, 1];
			let _: BitBox<AtomicU32, Msb0> =bitbox![AtomicU32, Msb0; 0, 1];
			let _: BitBox<RadiumU32, LocalBits> =bitbox![RadiumU32, LocalBits; 1; 100];
			let _: BitBox<RadiumU32, Lsb0> =bitbox![RadiumU32, Lsb0; 1; 100];
			let _: BitBox<RadiumU32, Msb0> =bitbox![RadiumU32, Msb0; 1; 100];
		}
		if atomic(size) {
			let _: BitBox<AtomicUsize, LocalBits> =bitbox![AtomicUsize, LocalBits; 0, 1];
			let _: BitBox<AtomicUsize, Lsb0> =bitbox![AtomicUsize, Lsb0; 0, 1];
			let _: BitBox<AtomicUsize, Msb0> =bitbox![AtomicUsize, Msb0; 0, 1];
			let _: BitBox<RadiumUsize, LocalBits> =bitbox![RadiumUsize, LocalBits; 1; 100];
			let _: BitBox<RadiumUsize, Lsb0> =bitbox![RadiumUsize, Lsb0; 1; 100];
			let _: BitBox<RadiumUsize, Msb0> =bitbox![RadiumUsize, Msb0; 1; 100];
		}
	}
	#[cfg(target_pointer_width = "64")]
	radium::if_atomic! {
		if atomic(64) {
			let _: BitBox<AtomicU64, LocalBits> =bitbox![AtomicU64, LocalBits; 0, 1];
			let _: BitBox<AtomicU64, Lsb0> =bitbox![AtomicU64, Lsb0; 0, 1];
			let _: BitBox<AtomicU64, Msb0> =bitbox![AtomicU64, Msb0; 0, 1];
			let _: BitBox<RadiumU64, LocalBits> =bitbox![RadiumU64, LocalBits; 1; 100];
			let _: BitBox<RadiumU64, Lsb0> =bitbox![RadiumU64, Lsb0; 1; 100];
			let _: BitBox<RadiumU64, Msb0> =bitbox![RadiumU64, Msb0; 1; 100];
		}
	}
}

#[test]
fn encode_bits() {
	let uint: GenericArray<u8, U1> =
		__encode_bits!(u8, Lsb0; 1, 0, 1, 0, 1, 1, 0, 0);
	assert_eq!(uint, arr![u8; 53]);

	let cell: GenericArray<Cell<u8>, U1> =
		__encode_bits!(Cell<u8>, Lsb0; 1, 0, 1, 0, 1, 1, 0, 0);
	assert_eq!(cell[0].get(), 53);

	let uint: GenericArray<u16, U1> = __encode_bits!(u16, Msb0;
		0, 1, 0, 0, 1, 0, 0, 0,
		0, 1, 1, 0, 1, 0, 0, 1
	);
	assert_eq!(uint, arr![u16; 0x4869]);

	let cell: GenericArray<Cell<u16>, U1> = __encode_bits!(Cell<u16>, Msb0;
		0, 1, 0, 0, 1, 0, 0, 0,
		0, 1, 1, 0, 1, 0, 0, 1
	);
	assert_eq!(cell[0].get(), 0x4869);

	let uint: GenericArray<u32, U1> = __encode_bits!(u32, LocalBits; 1, 0, 1);
	assert_eq!(uint.view_bits::<LocalBits>()[.. 3], bits![1, 0, 1]);

	let cell: GenericArray<Cell<u32>, U1> =
		__encode_bits!(Cell<u32>, LocalBits; 1, 0, 1);
	assert_eq!(cell.view_bits::<LocalBits>()[.. 3], bits![1, 0, 1]);
}

#[test]
fn make_elem() {
	let uint: u8 = __make_elem!(u8 as u8, Lsb0; 1, 0, 1, 0, 1, 1, 0, 0);
	assert_eq!(uint, 53);

	let cell: Cell<u8> =
		__make_elem!(Cell<u8> as u8, Lsb0; 1, 0, 1, 0, 1, 1, 0, 0);
	assert_eq!(cell.get(), 53);

	let uint: u16 = __make_elem!(u16 as u16, Msb0;
		0, 1, 0, 0, 1, 0, 0, 0,
		0, 1, 1, 0, 1, 0, 0, 1
	);
	assert_eq!(uint, 0x4869);

	let cell: Cell<u16> = __make_elem!(Cell<u16> as u16, Msb0;
		0, 1, 0, 0, 1, 0, 0, 0,
		0, 1, 1, 0, 1, 0, 0, 1
	);
	assert_eq!(cell.get(), 0x4869);

	let uint: u32 = __make_elem!(u32 as u32, LocalBits; 1, 0, 1);
	assert_eq!(uint.view_bits::<LocalBits>()[.. 3], bits![1, 0, 1]);

	let cell: Cell<u32> = __make_elem!(Cell<u32> as u32, LocalBits; 1, 0, 1);
	assert_eq!(cell.view_bits::<LocalBits>()[.. 3], bits![1, 0, 1]);

	/* `__make_elem!` is only invoked after `$ord` has already been made
	 * opaque to matchers as a single `:tt`. Invoking it directly with a path
	 * will fail the `:tt`, so this macro wraps it as one and forwards the
	 * rest.
	 */
	macro_rules! invoke_make_elem {
		(Cell<$typ:ident> as $sto:ident, $ord:path; $($rest:tt)*) => {
			__make_elem!(Cell<$typ> as $sto, $ord; $($rest)*)
		};
		($typ:ident as $sto:ident, $ord:path; $($rest:tt)*) => {
			__make_elem!($typ as $sto, $ord; $($rest)*)
		};
	}

	let uint: usize =
		invoke_make_elem!(usize as usize, crate::order::Lsb0; 0, 0, 1, 1);
	assert_eq!(uint, 12);

	let cell: Cell<usize> =
		invoke_make_elem!(Cell<usize> as usize, crate::order::Lsb0; 0, 0, 1, 1);
	assert_eq!(cell.get(), 12);
}
