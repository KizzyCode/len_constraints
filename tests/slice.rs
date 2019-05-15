use len_constraints::{
	slice::{ Fixed, Ranged, Relative },
	type_math::{ Sub, _4, _8 }
};
use std::convert::TryFrom;


macro_rules! s {
	($len:expr) => ([7; $len].as_ref());
}


#[test]
fn test_fixed() {
	Fixed::<u8, _4>::try_from(s!(4)).unwrap();
	Fixed::<u8, _8>::try_from(s!(8)).unwrap();
}
#[test]
fn test_fixed_err() {
	Fixed::<u8, _4>::try_from(s!(3)).unwrap_err();
	Fixed::<u8, _4>::try_from(s!(5)).unwrap_err();
	Fixed::<u8, _8>::try_from(s!(7)).unwrap_err();
	Fixed::<u8, _8>::try_from(s!(9)).unwrap_err();
}


#[test]
fn test_ranged() {
	Ranged::<u8, _4, _8>::try_from(s!(4)).unwrap();
	Ranged::<u8, _4, _8>::try_from(s!(7)).unwrap();
}
#[test]
fn test_ranged_err() {
	Ranged::<u8, _4, _8>::try_from(s!(3)).unwrap_err();
	Ranged::<u8, _4, _8>::try_from(s!(8)).unwrap_err();
}


#[test]
fn test_relative() {
	Relative::<u8, Sub, _4>::from(s!(4)).slice(8).unwrap();
	Relative::<u8, Sub, _8>::from(s!(9)).slice(17).unwrap();
}
#[test]
fn test_relative_err() {
	Relative::<u8, Sub, _4>::from(s!(0)).slice(3).unwrap_err();
	Relative::<u8, Sub, _4>::from(s!(3)).slice(8).unwrap_err();
	Relative::<u8, Sub, _8>::from(s!(9)).slice(16).unwrap_err();
}