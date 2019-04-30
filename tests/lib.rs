use len_constraints::{
	ConstraintViolation,
	type_math::{ Sub, _4, _8 }
};


#[test]
fn test_constraint_violation_fixed() {
	assert_eq!(ConstraintViolation::fixed::<_8>(7).by, -1);
	assert_eq!(ConstraintViolation::fixed::<_8>(9).by,  1);
}
#[test] #[should_panic]
fn test_constraint_violation_fixed_panic() {
	ConstraintViolation::fixed::<_8>(8);
}


#[test]
fn test_constraint_violation_range() {
	assert_eq!(ConstraintViolation::ranged::<_4, _8>(3).by, -1);
	assert_eq!(ConstraintViolation::ranged::<_4, _8>(8).by,  1);
}
#[test] #[should_panic]
fn test_constraint_violation_range_panic_0() {
	assert_eq!(ConstraintViolation::ranged::<_4, _8>(4).by, -1);
}
#[test] #[should_panic]
fn test_constraint_violation_range_panic_1() {
	assert_eq!(ConstraintViolation::ranged::<_4, _8>(7).by, 1);
}


#[test]
fn test_constraint_violation_relative() {
	assert_eq!(ConstraintViolation::relative::<Sub, _4>(2, 7).by, -1);
	assert_eq!(ConstraintViolation::relative::<Sub, _4>(4, 7).by,  1);
}
#[test] #[should_panic]
fn test_constraint_violation_relative_panic_0() {
	ConstraintViolation::relative::<Sub, _4>(0xDEAD, 3);
}
#[test] #[should_panic]
fn test_constraint_violation_relative_panic_1() {
	ConstraintViolation::relative::<Sub, _4>(3, 7);
}