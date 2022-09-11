#![doc = include_str!("../doc/array.md")]

use core::marker::PhantomData;

use generic_array::{
	ArrayLength,
	ConstDefault,
	GenericArray,
};

use crate::{
	mem,
	order::{
		BitOrder,
		Lsb0,
	},
	slice::BitSlice,
	store::BitStore,
	view::{
		BitView,
		BitViewSized,
	},
};

mod api;
mod iter;
mod ops;
mod tests;
mod traits;

pub use self::iter::IntoIter;

#[repr(transparent)]
#[doc = include_str!("../doc/array/BitArray.md")]
pub struct BitArray<S = usize, O = Lsb0, N = <S as BitStore>::Size>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	/// The ordering of bits within an `A::Store` element.
	pub _ord: PhantomData<O>,
	/// The wrapped data buffer.
	pub data: GenericArray<S, N::Output>,
}

impl<S, O, N> BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
	<N::Output as ArrayLength<S>>::ArrayType: ConstDefault,
{
	/// A bit-array with all bits initialized to zero.
	pub const ZERO: Self = Self {
		_ord: PhantomData,
		data: ConstDefault::DEFAULT,
	};
}

impl<S, O, N> BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	/// Wraps an existing buffer as a bit-array.
	///
	/// ## Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let data = arr![u16; 0, 1, 2, 3];
	/// let bits = BitArray::<_, Msb0, U64>::new(data);
	/// assert_eq!(bits.len(), 64);
	/// ```
	#[inline]
	pub const fn new(data: GenericArray<S, N::Output>) -> Self {
		Self {
			_ord: PhantomData,
			data,
		}
	}

	/// Removes the bit-array wrapper, returning the contained buffer.
	///
	/// ## Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	/// use generic_array::GenericArray;
	///
	/// let bits = bitarr![0; U30];
	/// let native: GenericArray<usize, U1> = bits.into_inner();
	/// ```
	#[inline]
	pub fn into_inner(self) -> GenericArray<S, N::Output> {
		self.data
	}

	/// Explicitly views the bit-array as a bit-slice.
	#[inline]
	pub fn as_bitslice(&self) -> &BitSlice<S, O> {
		&self.data.view_bits::<O>()[.. N::USIZE]
	}

	/// Explicitly views the bit-array as a mutable bit-slice.
	#[inline]
	pub fn as_mut_bitslice(&mut self) -> &mut BitSlice<S, O> {
		&mut self.data.view_bits_mut::<O>()[.. N::USIZE]
	}

	/// Views the bit-array as a slice of its underlying memory elements.
	#[inline]
	pub fn as_raw_slice(&self) -> &[S] {
		self.data.as_raw_slice()
	}

	/// Views the bit-array as a mutable slice of its underlying memory
	/// elements.
	#[inline]
	pub fn as_raw_mut_slice(&mut self) -> &mut [S] {
		self.data.as_raw_mut_slice()
	}

	/// Gets the length (in bits) of the bit-array.
	///
	/// This method is a compile-time constant.
	#[inline]
	pub fn len(&self) -> usize {
		N::USIZE
	}

	/// Tests whether the array is empty.
	///
	/// This method is a compile-time constant.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}
