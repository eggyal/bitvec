//! Additional trait implementations on bit-arrays.

use core::{
	borrow::{
		Borrow,
		BorrowMut,
	},
	cmp,
	convert::TryFrom,
	fmt::{
		self,
		Debug,
		Display,
		Formatter,
	},
	hash::{
		Hash,
		Hasher,
	},
	marker::Unpin,
};

use generic_array::{
	ArrayLength,
	GenericArray,
};
use tap::TryConv;

use super::BitArray;
use crate::{
	index::BitIdx,
	mem,
	order::BitOrder,
	slice::BitSlice,
	store::BitStore,
};

#[cfg(not(tarpaulin_include))]
impl<S, O, N> Borrow<BitSlice<S, O>> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn borrow(&self) -> &BitSlice<S, O> {
		self.as_bitslice()
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> BorrowMut<BitSlice<S, O>> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn borrow_mut(&mut self) -> &mut BitSlice<S, O> {
		self.as_mut_bitslice()
	}
}

impl<S, O, N> Clone for BitArray<S, O, N>
where
	S: BitStore + Clone,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn clone(&self) -> Self {
		Self::new(self.data.clone())
	}
}

impl<S, O, N> Eq for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> Ord for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.as_bitslice().cmp(other.as_bitslice())
	}
}

#[cfg(not(tarpaulin_include))]
impl<O1, N, S, O2, T> PartialEq<BitArray<S, O2, N>> for BitSlice<T, O1>
where
	O1: BitOrder,
	O2: BitOrder,
	S: BitStore,
	T: BitStore,
	N: mem::Elts<S>,
{
	#[inline]
	fn eq(&self, other: &BitArray<S, O2, N>) -> bool {
		self == other.as_bitslice()
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N, Rhs> PartialEq<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
	Rhs: ?Sized,
	BitSlice<S, O>: PartialEq<Rhs>,
{
	#[inline]
	fn eq(&self, other: &Rhs) -> bool {
		self.as_bitslice() == other
	}
}

#[cfg(not(tarpaulin_include))]
impl<N, S, T, O> PartialOrd<BitArray<S, O, N>> for BitSlice<T, O>
where
	S: BitStore,
	T: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn partial_cmp(&self, other: &BitArray<S, O, N>) -> Option<cmp::Ordering> {
		self.partial_cmp(other.as_bitslice())
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N, Rhs> PartialOrd<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
	Rhs: ?Sized,
	BitSlice<S, O>: PartialOrd<Rhs>,
{
	#[inline]
	fn partial_cmp(&self, other: &Rhs) -> Option<cmp::Ordering> {
		self.as_bitslice().partial_cmp(other)
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> AsRef<BitSlice<S, O>> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn as_ref(&self) -> &BitSlice<S, O> {
		self.as_bitslice()
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> AsMut<BitSlice<S, O>> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn as_mut(&mut self) -> &mut BitSlice<S, O> {
		self.as_mut_bitslice()
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> From<GenericArray<S, N::Output>> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn from(data: GenericArray<S, N::Output>) -> Self {
		Self::new(data)
	}
}

impl<S, O, N> TryFrom<&BitSlice<S, O>> for BitArray<S, O, N>
where
	S: BitStore + Clone,
	O: BitOrder,
	N: mem::Elts<S>,
{
	type Error = TryFromBitSliceError;

	#[inline]
	fn try_from(src: &BitSlice<S, O>) -> Result<Self, Self::Error> {
		src.try_conv::<&Self>().map(|this| this.clone())
	}
}

impl<S, O, N> TryFrom<&BitSlice<S, O>> for &BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	type Error = TryFromBitSliceError;

	#[inline]
	fn try_from(src: &BitSlice<S, O>) -> Result<Self, Self::Error> {
		TryFromBitSliceError::new::<S, O, N>(src).map(|()| unsafe {
			&*src
				.as_bitspan()
				.address()
				.to_const()
				.cast::<BitArray<S, O, N>>()
		})
	}
}

impl<S, O, N> TryFrom<&mut BitSlice<S, O>> for &mut BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	type Error = TryFromBitSliceError;

	#[inline]
	fn try_from(src: &mut BitSlice<S, O>) -> Result<Self, Self::Error> {
		TryFromBitSliceError::new::<S, O, N>(src).map(|()| unsafe {
			&mut *src
				.as_mut_bitspan()
				.address()
				.to_mut()
				.cast::<BitArray<S, O, N>>()
		})
	}
}

impl<S, O, N> Default for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
	GenericArray<S, N::Output>: Default,
{
	#[inline]
	fn default() -> Self {
		Self::new(Default::default())
	}
}

impl<S, O, N> Debug for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		self.as_bitspan().render(fmt, "Array", None)?;
		fmt.write_str(" ")?;
		Display::fmt(self, fmt)
	}
}

easy_fmt! {
	impl Binary
	impl Display
	impl LowerHex
	impl Octal
	impl UpperHex
	for BitArray
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> Hash for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
	#[inline]
	fn hash<H>(&self, hasher: &mut H)
	where H: Hasher {
		self.as_bitslice().hash(hasher);
	}
}

impl<S, O, N> Copy for BitArray<S, O, N>
where
	O: BitOrder,
	S: BitStore + Copy,
	N: mem::Elts<S>,
	<N::Output as ArrayLength<S>>::ArrayType: Copy,
{
}

impl<S, O, N> Unpin for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: mem::Elts<S>,
{
}

#[repr(transparent)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[doc = include_str!("../../doc/array/TryFromBitSliceError.md")]
pub struct TryFromBitSliceError(InnerError);

impl TryFromBitSliceError {
	/// Checks whether a bit-slice can be viewed as a bit-array.
	#[inline]
	fn new<S, O, N>(bits: &BitSlice<S, O>) -> Result<(), Self>
	where
		S: BitStore,
		O: BitOrder,
		N: mem::Elts<S>,
	{
		InnerError::new::<S, O, N>(bits).map_err(Self)
	}
}

impl Debug for TryFromBitSliceError {
	#[inline]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		fmt.write_str("TryFromBitSliceError::")?;
		match self.0 {
			InnerError::UnequalLen { actual, expected } => {
				write!(fmt, "UnequalLen({} != {})", actual, expected)
			},
			InnerError::Misaligned => fmt.write_str("Misaligned"),
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl Display for TryFromBitSliceError {
	#[inline]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		match self.0 {
			InnerError::UnequalLen { actual, expected } => write!(
				fmt,
				"bit-slice with length {} cannot be viewed as bit-array with \
				 length {}",
				actual, expected,
			),
			InnerError::Misaligned => fmt.write_str(
				"a bit-slice must begin at the front edge of a storage element \
				 in order to be viewed as a bit-array",
			),
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromBitSliceError {}

/// Opaque error type for bit-slice to bit-array view conversions.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum InnerError {
	/// A bit-slice did not match the length of the destination bit-array.
	UnequalLen {
		/// The length of the bit-slice that produced this error.
		actual:   usize,
		/// The length of the destination bit-array type.
		expected: usize,
	},
	/// A bit-slice did not begin at `BitIdx::MIN`.
	Misaligned,
}

impl InnerError {
	/// Checks whether a bit-slice is suitable to view as a bit-array.
	#[inline]
	fn new<S, O, N>(bits: &BitSlice<S, O>) -> Result<(), Self>
	where
		S: BitStore,
		O: BitOrder,
		N: mem::Elts<S>,
	{
		let bitspan = bits.as_bitspan();
		let actual = bitspan.len();
		let expected = N::USIZE;
		if actual != expected {
			return Err(Self::UnequalLen { actual, expected });
		}
		if bitspan.head() != BitIdx::<S::Mem>::MIN {
			return Err(Self::Misaligned);
		}
		Ok(())
	}
}
