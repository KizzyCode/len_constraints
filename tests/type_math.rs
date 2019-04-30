use len_constraints::type_math::*;


macro_rules! test_type_num {
	($name:ident) => ({
		// Remove leading underscore and parse value
		let value = stringify!($name).split_at(1).1;
		let value = usize::from_str_radix(value, 10).unwrap();
		
		// Validate name against value
		assert_eq!($name::VALUE, value);
	});
	($($name:ident),+) => ( $(test_type_num!($name);)+ )
}
#[test]
fn test_type_num() {
	test_type_num! {
		_0,
		_1, _2, _4, _8,
		_12, _16, _24, _32,
		_48, _64, _96, _128,
		_256, _384, _512, _1024,
		_2048, _4096, _8192, _16384,
		_32768, _65536
	}
}


#[test]
fn test_add() {
	assert_eq!(Add::r#do(3, 4).unwrap(), 7);
}
#[test]
fn test_add_err() {
	Add::r#do(usize::max_value(), 1).unwrap_err();
}


#[test]
fn test_sub() {
	assert_eq!(Sub::r#do(7, 4).unwrap(), 3);
}
#[test]
fn test_sub_err() {
	Sub::r#do(0, 1).unwrap_err();
}


#[test]
fn test_mul() {
	assert_eq!(Mul::r#do(3, 7).unwrap(), 21);
}
#[test]
fn test_mul_err() {
	Mul::r#do(usize::max_value(), 2).unwrap_err();
}


#[test]
fn test_div() {
	assert_eq!(Div::r#do(21, 7).unwrap(), 3);
}
#[test]
fn test_div_err() {
	Div::r#do(21, 0).unwrap_err();
}