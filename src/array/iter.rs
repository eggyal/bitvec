#![doc = include_str!("../../doc/array/iter.md")]

use core::{
	fmt::{
		self,
		Debug,
		Formatter,
	},
	iter::FusedIterator,
	ops::Range,
};

use tap::Pipe;
use wyz::comu::Const;

use super::BitArray;
use crate::{
	mem::Elts,
	order::BitOrder,
	ptr::BitPtr,
	slice::BitSlice,
	store::BitStore,
};

/// [Original](https://doc.rust-lang.org/std/primitive.array.html#impl-IntoIterator)
impl<S, O, N> IntoIterator for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	type IntoIter = IntoIter<S, O, N>;
	type Item = <IntoIter<S, O, N> as Iterator>::Item;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new(self)
	}
}

/// [Original](https://doc.rust-lang.org/std/primitive.array.html#impl-IntoIterator-1)
#[cfg(not(tarpaulin_include))]
impl<'a, S, O, N> IntoIterator for &'a BitArray<S, O, N>
where
	O: BitOrder,
	S: 'a + BitStore,
	N: Elts<S>,
{
	type IntoIter = <&'a BitSlice<S, O> as IntoIterator>::IntoIter;
	type Item = <&'a BitSlice<S, O> as IntoIterator>::Item;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.as_bitslice().into_iter()
	}
}

/// [Original](https://doc.rust-lang.org/std/primitive.array.html#impl-IntoIterator-2)
#[cfg(not(tarpaulin_include))]
impl<'a, S, O, N> IntoIterator for &'a mut BitArray<S, O, N>
where
	O: BitOrder,
	S: 'a + BitStore,
	N: Elts<S>,
{
	type IntoIter = <&'a mut BitSlice<S, O> as IntoIterator>::IntoIter;
	type Item = <&'a mut BitSlice<S, O> as IntoIterator>::Item;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.as_mut_bitslice().into_iter()
	}
}

#[doc = include_str!("../../doc/array/IntoIter.md")]
pub struct IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	/// The bit-array being iterated.
	array: BitArray<S, O, N>,
	/// The indices in `.array` that have not yet been yielded.
	///
	/// This range is always a strict subset of `0 .. self.array.len()`.
	alive: Range<usize>,
}

impl<S, O, N> Clone for IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitArray<S, O, N>: Clone,
{
	fn clone(&self) -> Self {
		Self {
			array: self.array.clone(),
			alive: self.alive.clone(),
		}
	}
}

impl<S, O, N> IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	/// Converts a bit-array into its iterator.
	///
	/// The [`.into_iter()`] method on bit-arrays forwards to this. While
	/// `BitArray` does deref to `&/mut BitSlice`, which also has
	/// `.into_iter()`, this behavior has always been present alongside
	/// `BitArray` and there is no legacy forwarding to preserve.
	///
	/// ## Original
	///
	/// [`IntoIter::new`](core::array::IntoIter::new)s
	#[inline]
	pub fn new(array: BitArray<S, O, N>) -> Self {
		Self {
			array,
			alive: 0 .. N::USIZE,
		}
	}

	/// Views the remaining unyielded bits in the iterator.
	///
	/// ## Original
	///
	/// [`IntoIter::as_slice`](core::array::IntoIter::as_slice)
	#[inline]
	pub fn as_bitslice(&self) -> &BitSlice<S, O> {
		unsafe { self.array.as_bitslice().get_unchecked(self.alive.clone()) }
	}

	#[inline]
	#[cfg(not(tarpaulin_include))]
	#[deprecated = "use `.as_bitslice()` instead"]
	#[allow(missing_docs, clippy::missing_docs_in_private_items)]
	pub fn as_slice(&self) -> &BitSlice<S, O> {
		self.as_bitslice()
	}

	/// Mutably views the remaining unyielded bits in the iterator.
	///
	/// ## Original
	///
	/// [`IntoIter::as_mut_slice`](core::array::IntoIter::as_mut_slice)
	#[inline]
	pub fn as_mut_bitslice(&mut self) -> &mut BitSlice<S, O> {
		unsafe {
			self.array
				.as_mut_bitslice()
				.get_unchecked_mut(self.alive.clone())
		}
	}

	#[inline]
	#[cfg(not(tarpaulin_include))]
	#[deprecated = "use `.as_bitslice_mut()` instead"]
	#[allow(missing_docs, clippy::missing_docs_in_private_items)]
	pub fn as_mut_slice(&mut self) -> &mut BitSlice<S, O> {
		self.as_mut_bitslice()
	}

	/// Gets a bit from the bit-array.
	#[inline]
	fn get(&self, index: usize) -> bool {
		unsafe {
			self.array
				.as_raw_slice()
				.pipe(BitPtr::<Const, S, O>::from_slice)
				.add(index)
				.read()
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> Debug for IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		fmt.debug_tuple("IntoIter")
			.field(&self.as_bitslice())
			.finish()
	}
}

impl<S, O, N> Iterator for IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	type Item = bool;

	easy_iter!();

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.alive.next().map(|idx| self.get(idx))
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.alive.nth(n).map(|idx| self.get(idx))
	}
}

impl<S, O, N> DoubleEndedIterator for IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.alive.next_back().map(|idx| self.get(idx))
	}

	#[inline]
	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.alive.nth_back(n).map(|idx| self.get(idx))
	}
}

impl<S, O, N> ExactSizeIterator for IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn len(&self) -> usize {
		self.alive.len()
	}
}

impl<S, O, N> FusedIterator for IntoIter<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
}
