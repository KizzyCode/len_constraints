use crate::{
	ConstraintViolation,
	type_math::{ TypeNum, Operator }
};
use std::{
	convert::TryFrom, error::Error, marker::PhantomData, slice::SliceIndex,
	ops::{ Deref, DerefMut, Index, IndexMut }
};


/// Implements the conversion traits
macro_rules! impl_conv {
	($type:ty where $($impl_args:tt)*) => {
		impl<$($impl_args)*> $type {
			/// The length of the constrained slice
			pub fn len(&self) -> usize {
				self.slice.len()
			}
			/// The constrained slice
			pub fn slice(&self) -> &[T] {
				self.slice
			}
			/// The mutable constrained slice
			pub fn slice_mut(&mut self) -> &mut[T] {
				self.slice
			}
		}
		impl<$($impl_args)*> Into<&'a[T]> for $type {
			fn into(self) -> &'a[T] {
				self.slice
			}
		}
		impl<$($impl_args)*> Into<&'a mut[T]> for $type {
			fn into(self) -> &'a mut[T] {
				self.slice
			}
		}
		impl<$($impl_args)*> Deref for $type {
			type Target = [T];
			fn deref(&self) -> &Self::Target {
				self.slice
			}
		}
		impl<$($impl_args)*> DerefMut for $type {
			fn deref_mut(&mut self) -> &mut Self::Target {
				self.slice
			}
		}
		impl<$($impl_args)*> AsRef<[T]> for $type {
			fn as_ref(&self) -> &[T] {
				self.slice
			}
		}
		impl<$($impl_args)*> AsMut<[T]> for $type {
			fn as_mut(&mut self) -> &mut[T] {
				self.slice
			}
		}
		impl<$($impl_args)*, I: SliceIndex<[T]>> Index<I> for $type {
			type Output = I::Output;
			fn index(&self, index: I) -> &Self::Output {
				&self.slice[index]
			}
		}
		impl<$($impl_args)*, I: SliceIndex<[T]>> IndexMut<I> for $type {
			fn index_mut(&mut self, index: I) -> &mut Self::Output {
				&mut self.slice[index]
			}
		}
	};
}


/// An immutable slice with a fixed length as length constraint
#[derive(Debug)]
pub struct FixedMut<'a, T, Val: TypeNum> {
	slice: &'a mut[T],
	constraint: PhantomData<Val>
}
impl<'a, T, Val: TypeNum> TryFrom<&'a mut[T]> for FixedMut<'a, T, Val> {
	type Error = Box<Error + 'static>;
	/// Validates `slice` against the length constraint and creates the constrained slice with it
	fn try_from(slice: &'a mut[T]) -> Result<Self, Self::Error> {
		match slice.len() {
			len if len == Val::VALUE => Ok(Self{ slice, constraint: PhantomData }),
			len => Err(ConstraintViolation::fixed::<Val>(len))?
		}
	}
}
impl_conv!(FixedMut<'a, T, Val> where 'a, T, Val: TypeNum);


/// An immutable slice with a range as length constraint
#[derive(Debug)]
pub struct RangedMut<'a, T, Start: TypeNum, End: TypeNum> {
	slice: &'a mut[T],
	constraint: PhantomData<(Start, End)>
}
impl<'a, T, Start: TypeNum, End: TypeNum> TryFrom<&'a mut[T]> for RangedMut<'a, T, Start, End> {
	type Error = Box<Error + 'static>;
	/// Validates `slice` against the length constraint and creates the constrained slice with it
	fn try_from(slice: &'a mut[T]) -> Result<Self, Self::Error> {
		match slice.len() {
			len if len >= Start::VALUE && len < End::VALUE =>
				Ok(Self{ slice, constraint: PhantomData }),
			len => Err(ConstraintViolation::ranged::<Start, End>(len))?
		}
	}
}
impl_conv!(RangedMut<'a, T, Start, End> where 'a, T, Start: TypeNum, End: TypeNum);


/// A mutable slice with a relative length as length constraint
///
/// _Note: Unlike the other constrained slices, this type does not validate the constraint on
/// construction but on deconstruction (`self.try_into()`)_
#[derive(Debug)]
pub struct RelativeMut<'a, T, Op: Operator, By: TypeNum> {
	slice: &'a mut[T],
	constraint: PhantomData<(Op, By)>
}
impl<'a, T, Op: Operator, By: TypeNum> RelativeMut<'a, T, Op, By> {
	/// Validates `slice` against the length constraint and creates the constrained slice with it
	pub fn try_from(slice: &'a mut[T], relative_to: usize) -> Result<Self, Box<Error + 'static>> {
		Self::validate(slice.len(), relative_to)?;
		Ok(Self::from(slice))
	}
	
	/// Computes the expected relative length from `relative_to`, validates the wrapped `slice`
	/// against in and returns it on success
	pub fn slice(self, relative_to: usize) -> Result<&'a[T], Box<Error + 'static>> {
		Self::validate(self.slice.len(), relative_to)?;
		Ok(self.slice)
	}
	/// Computes the expected relative length from `relative_to`, validates the wrapped `slice`
	/// against in and returns it on success
	pub fn slice_mut(self, relative_to: usize) -> Result<&'a mut[T], Box<Error + 'static>> {
		Self::validate(self.slice.len(), relative_to)?;
		Ok(self.slice)
	}
	
	/// Validates that `len` is valid relative to `relative_to`
	fn validate(len: usize, relative_to: usize) -> Result<(), Box<Error + 'static>> {
		let expected = Op::r#do(relative_to, By::VALUE)?;
		match len == expected {
			true => Ok(()),
			false => Err(ConstraintViolation::relative::<Op, By>(len, relative_to))?
		}
	}
}
impl<'a, T, Op: Operator, By: TypeNum> From<&'a mut[T]> for RelativeMut<'a, T, Op, By> {
	/// Creates a new relative constrained slice
	fn from(slice: &'a mut[T]) -> Self {
		RelativeMut{ slice, constraint: PhantomData }
	}
}