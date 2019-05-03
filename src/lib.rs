//! ## About
//! This crate implements traits and types that allows you to implement type-pinned length
//! constraints in your API.
//!
//!
//! ## Why?
//! How often have you seen APIs like this?
//! ```ignore
//! // BAD EXAMPLE!
//!
//! fn encrypt(buf: &mut[u8], plaintext: &[u8], key: &[u8], nonce: &[u8])
//! 	-> Result<usize, Box<dyn Error + 'static>>
//! {
//! 	// Validate parameters
//! 	if plaintext.len() > 65_635 { Err("Plaintext is too large")? }
//! 	if buf.len() < plaintext.len() + 16 { Err("Buffer is smaller than plaintext length")? }
//! 	if key.len() != 32 { Err("Invalid key size")? }
//! 	if nonce.len() != 12 { Err("Invalid nonce size")? }
//!
//! 	// Do sth.
//! 	unimplemented!()
//! }
//! ```
//! As you can see, this API is pretty opaque and requires a lot of manual checks.
//!
//! Of course s.o. could use array references:
//! ```ignore
//! // MEH EXAMPLE...
//!
//! fn encrypt(buf: &mut[u8], plaintext: &[u8], key: &[u8; 32], nonce: &[u8; 12])
//! 	-> Result<usize, Box<dyn Error + 'static>>
//! {
//! 	// Validate parameters
//! 	if plaintext.len() > 65_635 { Err("Plaintext is too large")? }
//! 	if buf.len() < plaintext.len() + 16 { Err("Buffer is smaller than plaintext length")? }
//!
//! 	// Do sth.
//! 	unimplemented!()
//! }
//! ```
//! But array references also have their disadvantages. They are not suitable for multiple valid
//! lengths (allow anything in `16..=32`) nor can they represent relative relationships. Also
//! converting between other data types and arrays can get annoying.
//!
//! `len_constraints` tries to solve this problem:
//! ```
//! // GOOD EXAMPLE :D
//!
//! use std::{ convert::TryInto, error::Error };
//! use len_constraints::{
//! 	slice_mut::RelativeMut, slice::{ Fixed, Ranged },
//! 	type_math::{ Add, _0, _12, _16, _32, _65536 }
//! };
//!
//! fn encrypt(buf: RelativeMut<u8, Add, _16>, plaintext: Ranged<u8, _0, _65536>,
//! 	key: Fixed<u8, _32>, nonce: Fixed<u8, _12>) -> Result<usize, Box<Error + 'static>>
//! {
//! 	// Get buffer (we do this here because there may not be a relationship at an earlier stage)
//! 	let buf = buf.get_slice_mut(plaintext.len())?;
//!
//! 	// Do sth.
//! 	Ok(7)
//! }
//!
//! fn main() -> Result<(), Box<Error + 'static>> {
//! 	// Parameters
//! 	let mut buf: &mut[u8] = &mut[0; 9 + 16];
//! 	let plaintext: &[u8] = b"Testolope";
//! 	let key: &[u8] = &[0; 32];
//! 	let nonce = "12 byte Nonc".as_bytes();
//!
//! 	// Call function
//! 	encrypt(buf.into(), plaintext.try_into()?, key.try_into()?, nonce.try_into()?)?;
//! 	Ok(())
//! }
//! ```
//! As you can see, we now can describe complex relationships in the function signature – this makes
//! the API more transparent and removes the need for manual (and error-prone) parameter validation.


/// The `TypeNum` and `Operator` traits which can be used as type arguments as well as some
/// predefined numbers and operators.
#[macro_use] pub mod type_math;
/// Some wrappers for immutable slices with various length constraints
pub mod slice;
/// Some wrappers for mutable slices with various length constraints
pub mod slice_mut;
#[macro_use] mod constraint_macro;

pub use self::type_math::{ TypeNum, Operator };
use std::{
	convert::TryFrom, error::Error,
	fmt::{ self, Display, Formatter }
};


/// A constraint violation error
#[derive(Debug)]
pub struct ConstraintViolation {
	#[doc(hidden)]
	pub constraint: String,
	#[doc(hidden)]
	pub by: i128
}
impl ConstraintViolation {
	/// Creates a new error in case a fixed constraint was violated
	pub fn fixed<Val: TypeNum>(len: usize) -> Self {
		// Compute diff
		let by = i128::try_from(len).unwrap() - i128::try_from(Val::VALUE).unwrap();
		assert_ne!(by, 0, "Cannot construct `ConstraintViolation` for valid constraint");
		
		Self{ constraint: format!("{:?}", Val::default()), by }
	}
	/// Creates a new error in case a range constraint was violated
	pub fn ranged<Start: TypeNum, End: TypeNum>(len: usize) -> Self {
		// Prepare values
		let len = i128::try_from(len).unwrap();
		let (start, end) =
			(i128::try_from(Start::VALUE).unwrap(), i128::try_from(End::VALUE).unwrap());
		
		// Compute diff
		let by = match (start, end) {
			(start, _) if len < start => len - start,
			(_, end) if len >= end => len - (end - 1),
			_ => panic!("Cannot construct `ConstraintViolation` for valid constraint")
		};
		
		Self{ constraint: format!("Range<{:?} .. {:?}>", Start::default(), End::default()), by }
	}
	/// Creates a new error in case a relative constraint was violated
	pub fn relative<Op: Operator, By: TypeNum>(len: usize, other: usize) -> Self {
		// Compute the absolute length of the relative constraint
		let abs_len = Op::r#do(other, By::VALUE)
			.expect("Cannot construct `ConstraintViolation` for illegal constraint");
		
		// Compute diff
		let by = i128::try_from(len).unwrap() - i128::try_from(abs_len).unwrap();
		assert_ne!(by, 0, "Cannot construct `ConstraintViolation` for valid constraint");
		
		Self{ constraint: format!("{:?}({:?})", Op::default(), By::default()), by }
	}
}
impl Display for ConstraintViolation {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let sign = match self.by.is_positive() {
			true => "+",
			false => ""
		};
		write!(f, "The length constraint `{}` was violated by {}{}", self.constraint, sign, self.by)
	}
}
impl Error for ConstraintViolation {}


