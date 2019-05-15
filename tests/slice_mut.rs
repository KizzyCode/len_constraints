use len_constraints::{
	slice_mut::{ FixedMut, RangedMut, RelativeMut },
	type_math::{ Sub, _4, _8 }
};
use std::convert::TryFrom;


macro_rules! s {
	($len:expr) => ([7; $len].as_mut());
}


#[test]
fn test_fixed_mut() {
	FixedMut::<u8, _4>::try_from(s!(4)).unwrap();
	FixedMut::<u8, _8>::try_from(s!(8)).unwrap();
}
#[test]
fn test_fixed_mut_err() {
	FixedMut::<u8, _4>::try_from(s!(3)).unwrap_err();
	FixedMut::<u8, _4>::try_from(s!(5)).unwrap_err();
	FixedMut::<u8, _8>::try_from(s!(7)).unwrap_err();
	FixedMut::<u8, _8>::try_from(s!(9)).unwrap_err();
}


#[test]
fn test_ranged_mut() {
	RangedMut::<u8, _4, _8>::try_from(s!(4)).unwrap();
	RangedMut::<u8, _4, _8>::try_from(s!(7)).unwrap();
}
#[test]
fn test_ranged_mut_err() {
	RangedMut::<u8, _4, _8>::try_from(s!(3)).unwrap_err();
	RangedMut::<u8, _4, _8>::try_from(s!(8)).unwrap_err();
}


#[test]
fn test_relative_mut() {
	RelativeMut::<u8, Sub, _4>::from(s!(4)).slice_mut(8).unwrap();
	RelativeMut::<u8, Sub, _8>::from(s!(9)).slice_mut(17).unwrap();
}
#[test]
fn test_relative_mut_err() {
	RelativeMut::<u8, Sub, _4>::from(s!(0)).slice_mut(3).unwrap_err();
	RelativeMut::<u8, Sub, _4>::from(s!(3)).slice_mut(8).unwrap_err();
	RelativeMut::<u8, Sub, _8>::from(s!(9)).slice_mut(16).unwrap_err();
}