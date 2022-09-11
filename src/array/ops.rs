//! Operator trait implementations for bit-arrays.

use core::ops::{
	BitAnd,
	BitAndAssign,
	BitOr,
	BitOrAssign,
	BitXor,
	BitXorAssign,
	Deref,
	DerefMut,
	Index,
	IndexMut,
	Not,
};

use super::BitArray;
use crate::{
	mem::Elts,
	order::BitOrder,
	slice::BitSlice,
	store::BitStore,
};

#[cfg(not(tarpaulin_include))]
impl<S, O, N> BitAndAssign<BitArray<S, O, N>> for BitSlice<S, O>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn bitand_assign(&mut self, rhs: BitArray<S, O, N>) {
		*self &= rhs.as_bitslice()
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> BitAndAssign<&BitArray<S, O, N>> for BitSlice<S, O>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn bitand_assign(&mut self, rhs: &BitArray<S, O, N>) {
		*self &= rhs.as_bitslice()
	}
}

impl<S, O, N, Rhs> BitAnd<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: BitAndAssign<Rhs>,
{
	type Output = Self;

	#[inline]
	fn bitand(mut self, rhs: Rhs) -> Self::Output {
		self &= rhs;
		self
	}
}

impl<S, O, N, Rhs> BitAndAssign<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: BitAndAssign<Rhs>,
{
	#[inline]
	fn bitand_assign(&mut self, rhs: Rhs) {
		*self.as_mut_bitslice() &= rhs;
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> BitOrAssign<BitArray<S, O, N>> for BitSlice<S, O>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn bitor_assign(&mut self, rhs: BitArray<S, O, N>) {
		*self |= rhs.as_bitslice()
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> BitOrAssign<&BitArray<S, O, N>> for BitSlice<S, O>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn bitor_assign(&mut self, rhs: &BitArray<S, O, N>) {
		*self |= rhs.as_bitslice()
	}
}

impl<S, O, N, Rhs> BitOr<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: BitOrAssign<Rhs>,
{
	type Output = Self;

	#[inline]
	fn bitor(mut self, rhs: Rhs) -> Self::Output {
		self |= rhs;
		self
	}
}

impl<S, O, N, Rhs> BitOrAssign<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: BitOrAssign<Rhs>,
{
	#[inline]
	fn bitor_assign(&mut self, rhs: Rhs) {
		*self.as_mut_bitslice() |= rhs;
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> BitXorAssign<BitArray<S, O, N>> for BitSlice<S, O>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn bitxor_assign(&mut self, rhs: BitArray<S, O, N>) {
		*self ^= rhs.as_bitslice()
	}
}

#[cfg(not(tarpaulin_include))]
impl<S, O, N> BitXorAssign<&BitArray<S, O, N>> for BitSlice<S, O>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn bitxor_assign(&mut self, rhs: &BitArray<S, O, N>) {
		*self ^= rhs.as_bitslice()
	}
}

impl<S, O, N, Rhs> BitXor<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: BitXorAssign<Rhs>,
{
	type Output = Self;

	#[inline]
	fn bitxor(mut self, rhs: Rhs) -> Self::Output {
		self ^= rhs;
		self
	}
}

impl<S, O, N, Rhs> BitXorAssign<Rhs> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: BitXorAssign<Rhs>,
{
	#[inline]
	fn bitxor_assign(&mut self, rhs: Rhs) {
		*self.as_mut_bitslice() ^= rhs;
	}
}

impl<S, O, N> Deref for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	type Target = BitSlice<S, O>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.as_bitslice()
	}
}

impl<S, O, N> DerefMut for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_bitslice()
	}
}

impl<S, O, N, Idx> Index<Idx> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: Index<Idx>,
{
	type Output = <BitSlice<S, O> as Index<Idx>>::Output;

	#[inline]
	fn index(&self, index: Idx) -> &Self::Output {
		&self.as_bitslice()[index]
	}
}

impl<S, O, N, Idx> IndexMut<Idx> for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
	BitSlice<S, O>: IndexMut<Idx>,
{
	#[inline]
	fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
		&mut self.as_mut_bitslice()[index]
	}
}

impl<S, O, N> Not for BitArray<S, O, N>
where
	S: BitStore,
	O: BitOrder,
	N: Elts<S>,
{
	type Output = Self;

	#[inline]
	fn not(mut self) -> Self::Output {
		for elem in self.as_raw_mut_slice() {
			elem.store_value(!elem.load_value());
		}
		self
	}
}
