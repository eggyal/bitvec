#![allow(deprecated)]
#![doc = include_str!("../doc/macros.md")]

#[macro_use]
#[doc(hidden)]
pub mod internal;

mod tests;

#[macro_export]
#[doc = include_str!("../doc/macros/BitArr_type.md")]
macro_rules! BitArr {
	(for $len:ty, in $store:ty, $order:ty $(,)?) => {
		$crate::array::BitArray::<$store, $order, $len>
	};

	(for $len:ty, in $store:ty $(,)?) => {
		$crate::BitArr!(for $len, in $store, $crate::order::Lsb0)
	};

	(for $len:ty) => {
		$crate::BitArr!(for $len, in usize)
	};
}

#[macro_export]
#[doc = include_str!("../doc/macros/bitarr_value.md")]
macro_rules! bitarr {
	/* `const`-expression constructors.
	 *
	 * These arms expand to expressions which are guaranteed to be valid in
	 * `const` position: initializing `static` or `const`, or arguments to
	 * `const fn`.
	 *
	 * > Other arms *may* be valid in `const`s, but do not guarantee it.
	 *
	 * They are more restricted than the general variants below, because the
	 * trait system is not yet usable in `const` contexts and thus these
	 * expansions can only use codepaths defined in this module, and cannot use
	 * the rest of `bitvec`’s systems.
	 *
	 * All valid invocations with a leading `const` will remain valid if the
	 * `const` is removed, though their expansion may change to no longer be
	 * valid in `const` contexts.
	 */

	//  Bit-sequencing requires detecting `Cell` separately from other types.
	//  See below.

	(const Cell<$store:ident>, $order:ident; $($val:expr),* $(,)?) => {{
		use $crate::macros::internal::generic_array::GenericArray;

		type Len = $crate::__count_ty!($($val),*);
		type Elts = <Len as $crate::mem::Elts<$store>>::Output;
		type Data = GenericArray<Cell<$store>, Elts>;
		const DATA: Data = $crate::__encode_bits!(Cell<$store>, $order; $($val),*);

		type This = $crate::array::BitArray<Cell<$store>, $order, Len>;
		This::new(DATA)
	}};
	(const $store:ident, $order:ident; $($val:expr),* $(,)?) => {{
		use $crate::macros::internal::generic_array::GenericArray;

		type Len = $crate::__count_ty!($($val),*);
		type Elts = <Len as $crate::mem::Elts<$store>>::Output;
		type Data = GenericArray<$store, Elts>;
		const DATA: Data = $crate::__encode_bits!($store, $order; $($val),*);

		type This = $crate::array::BitArray<$store, $order, Len>;
		This::new(DATA)
	}};

	//  Bit-repetition is agnostic to types, so it only needs two arms.

	(const $store:ty, $order:ty; $val:expr; $len:ty) => {{
		use $crate::macros::internal::generic_array::{arr, GenericArray};
		type Mem = <$store as $crate::store::BitStore>::Mem;

		type Elts = <$len as $crate::mem::Elts<$store>>::Output;
		const ELEM: Mem = $crate::__extend_bool!($val, $store);
		const DATA: GenericArray<Mem, Elts> = arr![Mem; ELEM; Elts];

		type This = $crate::array::BitArray<$store, $order, $len>;
		unsafe { core::mem::transmute::<_, This>(DATA) }
	}};
	(const $val:expr; $len:ty) => {{
		$crate::bitarr!(const usize, $crate::order::Lsb0; $val; $len)
	}};

	(const $($val:expr),* $(,)?) => {{
		$crate::bitarr!(const usize, Lsb0; $($val),*)
	}};

	/* Non-`const` constructors.
	 *
	 * These expansions are allowed to produce code that does not run in `const`
	 * contexts. While it is *likely* that the expansions will be evaluated at
	 * compile-time, they won’t do so while the `const` engine is active.
	 */

	/* Bit-sequence encoding.
	 *
	 * This requires four arms to the `const` section’s one, because of how both
	 * the ordering and storage arguments may be provided. As macros operate
	 * syntactically, before the type system begins, they have to accept any
	 * syntax that could later be accepted as the name of a satisfying type.
	 *
	 * The `$order:ident` matcher uses the fact that `:ident` matches remain
	 * matchable across deeper macro invocations, so that the bottom of the
	 * macro stack can detect the magic tokens `LocalBits`, `Lsb0`, and `Msb0`,
	 * and operate accordingly. The `$order:path` matcher is always opaque, and
	 * serves as a fallback for complex type-names.
	 *
	 * `Cell<$store>` uses literal detection to extract the interior type width.
	 * This cannot be done by `:ty` or `:path`, as these are opaque, and
	 * `:ident` does not match `Cell<_>`.
	 */

	(Cell<$store:ident>, $order:ident; $($val:expr),* $(,)?) => {{
		use $crate::macros::internal::core;
		type Celled = core::cell::Cell<$store>;

		type Len = $crate::__count_ty!($($val),*);
		type This = $crate::array::BitArray<Celled, $order, Len>;

		This::new($crate::__encode_bits!(Cell<$store>, $order; $($val),*))
	}};
	(Cell<$store:ident>, $order:path; $($val:expr),* $(,)?) => {{
		use $crate::macros::internal::core;
		type Celled = core::cell::Cell<$store>;

		type Len = $crate::__count_ty!($($val),*);
		type This = $crate::array::BitArray<Celled, $order, Len>;

		This::new($crate::__encode_bits!(Cell<$store>, $order; $($val),*))
	}};

	($store:ident, $order:ident; $($val:expr),* $(,)?) => {{
		type Len = $crate::__count_ty!($($val),*);
		type This = $crate::array::BitArray<$store, $order, Len>;

		This::new($crate::__encode_bits!($store, $order; $($val),*))
	}};
	($store:ident, $order:path; $($val:expr),* $(,)?) => {{
		type Len = $crate::__count_ty!($($val),*);
		type This = $crate::array::BitArray<$store, $order, Len>;

		This::new($crate::__encode_bits!($store, $order; $($val),*))
	}};


	($store:ty, $order:ty; $val:expr; $len:ty) => {{
		$crate::bitarr!(const $store, $order; $val; $len)
	}};
	($val:expr; $len:ty) => {{
		$crate::bitarr!(const $val; $len)
	}};
	($($val:expr),* $(,)?) => {
		$crate::bitarr!(usize, Lsb0; $($val),*)
	};
}

#[macro_export]
#[doc = include_str!("../doc/macros/bits.md")]
macro_rules! bits {
	/* `&'static` constructors.
	 *
	 * Like the `bitarr!(const …)` arms, these arms must expand to code that is
	 * valid in `const` contexts. As such, they can only accept `$order`
	 * arguments that are one of the `LocalBits`, `Lsb0`, or `Msb0` literals.
	 * Once the underlying `static BitArray` is created,
	 */
	(static mut Cell<$store:ident>, $order:ty; $val:expr; $len:ty) => {{
		use $crate::macros::internal::{core, generic_array::typenum::Unsigned};
		type Celled = core::cell::Cell<$store>;
		static mut DATA: $crate::BitArr!(for $len, in Celled, $order) =
			$crate::bitarr!(const Cell<$store>, $order; $val; $len);
		 &mut DATA[.. <$len as Unsigned>::USIZE]
	}};
	(static mut $store:ident, $order:ident; $val:expr; $len:ty) => {{
		static mut DATA: $crate::BitArr!(for $len, in $store, $order) =
			$crate::bitarr!(const $store, $order; $val; $len);
		DATA.get_unchecked_mut(.. <$len as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE)
	}};

	(static mut Cell<$store:ident>, $order:ident; $($val:expr),* $(,)?) => {{
		use $crate::macros::internal::{core, generic_array::typenum::Unsigned};
		type Celled = core::cell::Cell<$store>;
		type Bits = $crate::__count_ty!($($val),*);

		static mut DATA: $crate::BitArr!(for Bits, in $store, $order) =
			$crate::bitarr!(const $store, $order; $($val),*);
		&mut *(
			DATA.get_unchecked_mut(.. <Bits as Unsigned>::USIZE)
				as *mut $crate::slice::BitSlice<$store, $order>
				as *mut $crate::slice::BitSlice<Celled, $order>
		)
	}};
	(static mut $store:ident, $order:ident; $($val:expr),* $(,)?) => {{
		type Bits = $crate::__count_ty!($($val),*);
		static mut DATA: $crate::BitArr!(for Bits, in $store, $order) =
			$crate::bitarr!(const $store, $order; $($val),*);
		DATA.get_unchecked_mut(.. <Bits as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE)
	}};

	(static mut $val:expr; $len:ty) => {{
		static mut DATA: $crate::BitArr!(for $len) =
			$crate::bitarr!(const usize, $crate::order::Lsb0; $val; $len);
		DATA.get_unchecked_mut(.. <$len as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE)
	}};
	(static mut $($val:expr),* $(,)?) => {{
		$crate::bits!(static mut usize, Lsb0; $($val),*)
	}};

	(static Cell<$store:ident>, $order:ty; $val:expr; $len:ty) => {{
		use $crate::macros::internal::{core, generic_array::typenum::Unsigned};
		type Celled = core::cell::Cell<$store>;
		static DATA: $crate::BitArr!(for $len, in $store, $order) =
			$crate::bitarr!(const $store, $order; $val; $len);
		unsafe {
			&*(
				DATA.get_unchecked(.. <$len as Unsigned>::USIZE)
					as *const $crate::slice::BitSlice<$store, $order>
					as *const $crate::slice::BitSlice<Celled, $order>
			)
		}
	}};
	(static Cell<$store:ident>, $order:ident; $($val:expr),* $(,)?) => {{
		use $crate::macros::internal::{core, generic_array::typenum::Unsigned};
		type Celled = core::cell::Cell<$store>;
		type Bits = $crate::__count_ty!($($val),*);

		static DATA: $crate::BitArr!(for Bits, in $store, $order) =
			$crate::bitarr!(const $store, $order; $($val),*);
		unsafe {
			&*(
				DATA.get_unchecked(.. <Bits as Unsigned>::USIZE)
					as *const $crate::slice::BitSlice<$store, $order>
					as *const $crate::slice::BitSlice<Celled, $order>
			)
		}
	}};

	(static $store:ident, $order:ident; $val:expr; $len:ty) => {{
		static DATA: $crate::BitArr!(for $len, in $store, $order) =
			$crate::bitarr!(const $store, $order; $val; $len);
		unsafe { DATA.get_unchecked(.. <$len as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE) }
	}};
	(static $val:expr; $len:ty) => {{
		static DATA: $crate::BitArr!(for $len) =
			$crate::bitarr!(const usize, $crate::order::Lsb0; $val; $len);
		unsafe { DATA.get_unchecked(.. <$len as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE) }
	}};

	(static $store:ident, $order:ident; $($val:expr),* $(,)?) => {{
		type Bits = $crate::__count_ty!($($val),*);
		static DATA: $crate::BitArr!(for Bits, in $store, $order) =
			$crate::bitarr!(const $store, $order; $($val),*);
		unsafe { DATA.get_unchecked(.. <Bits as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE) }
	}};
	(static $($val:expr),* $(,)?) => {{
		$crate::bits!(static usize, Lsb0; $($val),*)
	}};

	//  Repetition syntax `[bit ; count]`.
	//  NOTE: `count` must be a `const`, as this is a non-allocating macro.

	//  Sequence syntax `[bit (, bit)*]` or `[(bit ,)*]`.

	//  Explicit order and store.

	(mut Cell<$store:ident>, $order:ident; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&mut $crate::bitarr!(Cell<$store>, $order; $($val),*)[.. BITS]
	}};
	(mut Cell<$store:ident>, $order:path; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&mut $crate::bitarr!(Cell<$store>, $order; $($val),*)[.. BITS]
	}};

	(mut $store:ident, $order:ident; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&mut $crate::bitarr!($store, $order; $($val),*)[.. BITS]
	}};
	(mut $store:ident, $order:path; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&mut $crate::bitarr!($store, $order; $($val),*)[.. BITS]
	}};

	//  Explicit order and store.
	(mut $store:ty, $order:ty; $val:expr; $len:ty) => {{
		&mut $crate::bitarr!($store, $order; $val; $len)[.. <$len as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE]
	}};
	//  Default order and store.
	(mut $val:expr; $len:ty) => {
		$crate::bits!(mut usize, $crate::order::Lsb0; $val; $len)
	};

	//  Default order and store.
	(mut $($val:expr),* $(,)?) => {
		$crate::bits!(mut usize, Lsb0; $($val),*)
	};

	//  Repeat everything from above, but now immutable.

	($store:ty, $order:ty; $val:expr; $len:ty) => {{
		&$crate::bitarr!($store, $order; $val; $len)[.. <$len as $crate::macros::internal::generic_array::typenum::Unsigned>::USIZE]
	}};

	(Cell<$store:ident>, $order:ident; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&$crate::bitarr!(Cell<$store>, $order; $($val),*)[.. BITS]
	}};
	($store:ident, $order:ident; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&$crate::bitarr!($store, $order; $($val),*)[.. BITS]
	}};

	(Cell<$store:ident>, $order:path; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&$crate::bitarr!(Cell<$store>, $order; $($val),*)[.. BITS]
	}};
	($store:ident, $order:path; $($val:expr),* $(,)?) => {{
		const BITS: usize = $crate::__count!($($val),*);
		&$crate::bitarr!($store, $order; $($val),*)[.. BITS]
	}};

	//  Default order and store.
	($val:expr; $len:ty) => {
		$crate::bits!(usize, $crate::order::Lsb0; $val; $len)
	};
	($($val:expr),* $(,)?) => {
		$crate::bits!(usize, Lsb0; $($val),*)
	};
}

#[macro_export]
#[cfg(feature = "alloc")]
#[doc = include_str!("../doc/macros/bitvec.md")]
macro_rules! bitvec {
	//  First, capture the repetition syntax, as it is permitted to use runtime
	//  values for the repetition count.
	($store:ty, $order:ty; $val:expr; $len:expr) => {
		$crate::vec::BitVec::<$store, $order>::repeat($val != 0, $len)
	};
	// Capture `Cell<T>` patterns and prevent them from being parsed as
	// comparisons. Guess we didn't escape Most Vexing Parse after all.
	(Cell<$store:ident>, $order:ident $($rest:tt)*) => {
		$crate::vec::BitVec::from_bitslice($crate::bits!(Cell<$store>, $order $($rest)*))
	};
	($val:expr; $len:expr) => {
		$crate::bitvec!(usize, $crate::order::Lsb0; $val; $len)
	};

	//  Delegate all others to the `bits!` macro.
	($($arg:tt)*) => {
		$crate::vec::BitVec::from_bitslice($crate::bits!($($arg)*))
	};
}

#[macro_export]
#[cfg(feature = "alloc")]
#[doc = include_str!("../doc/macros/bitbox.md")]
macro_rules! bitbox {
	($($arg:tt)*) => {
		$crate::bitvec!($($arg)*).into_boxed_bitslice()
	};
}
