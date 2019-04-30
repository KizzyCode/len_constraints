use std::{ error::Error, fmt::Debug };


/// A type representing a number that can be used as a generic type argument
pub trait TypeNum: Debug + Default {
	/// The value represented by this type
	const VALUE: usize;
	
	/// The value represented by this type
	fn value() -> usize {
		Self::VALUE
	}
}
/// Creates a new type number representing `$value` with `$name` as identifier
#[macro_export]
macro_rules! type_num {
	($value:expr => $name:ident) => (
		/// A type representing a number that can be used as a generic type argument
		#[derive(Debug, Default)]
		pub struct $name;
		impl $crate::type_math::TypeNum for $name {
			const VALUE: usize = $value;
		}
	);
	($($value:expr => $name:ident),+) => ( $(type_num!($value => $name);)+ )
}
type_num! {
	0 => _0,
	1 => _1, 2 => _2, 4 => _4, 8 => _8,
	12 => _12, 16 => _16, 24 => _24, 32 => _32,
	48 => _48, 64 => _64, 96 => _96, 128 => _128,
	256 => _256, 384 => _384, 512 => _512, 1024 => _1024,
	2048 => _2048, 4096 => _4096, 8192 => _8192, 16384 => _16384,
	32768 => _32768, 65536 => _65536
}


/// A type representing an operator that can be used as a generic type argument
pub trait Operator: Debug + Default {
	/// Performs the operation represented by this type between `a` and `b`
	fn r#do(a: usize, b: usize) -> Result<usize, Box<Error + 'static>>;
}

/// An operator representing an addition
#[derive(Debug, Default)]
pub struct Add;
impl Operator for Add {
	fn r#do(a: usize, b: usize) -> Result<usize, Box<Error + 'static>> {
		Ok(a.checked_add(b).ok_or("Integer overflow")?)
	}
}

/// An operator representing a subtraction
#[derive(Debug, Default)]
pub struct Sub;
impl Operator for Sub {
	fn r#do(a: usize, b: usize) -> Result<usize, Box<Error + 'static>> {
		Ok(a.checked_sub(b).ok_or("Integer underflow")?)
	}
}

/// An operator representing a multiplication
#[derive(Debug, Default)]
pub struct Mul;
impl Operator for Mul {
	fn r#do(a: usize, b: usize) -> Result<usize, Box<Error + 'static>> {
		Ok(a.checked_mul(b).ok_or("Integer overflow")?)
	}
}

/// An operator representing a division
#[derive(Debug, Default)]
pub struct Div;
impl Operator for Div {
	fn r#do(a: usize, b: usize) -> Result<usize, Box<Error + 'static>> {
		Ok(a.checked_div(b).ok_or("Division by zero")?)
	}
}