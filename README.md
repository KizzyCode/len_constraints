[![docs.rs](https://docs.rs/len_constraints/badge.svg)](https://docs.rs/len_constraints)
[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/len_constraints.svg)](https://crates.io/crates/len_constraints)
[![Download numbers](https://img.shields.io/crates/d/len_constraints.svg)](https://crates.io/crates/len_constraints)
[![Travis CI](https://travis-ci.org/KizzyCode/len_constraints.svg?branch=master)](https://travis-ci.org/KizzyCode/len_constraints)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/len_constraints?svg=true)](https://ci.appveyor.com/project/KizzyCode/len-constraints)
[![dependency status](https://deps.rs/crate/len_constraints/0.1.0/status.svg)](https://deps.rs/crate/len_constraints/0.1.0)


# len_constraints
Welcome to `len_constraints` 🎉


## About
This crate implements traits and types that allows you to implement type-pinned length constraints
in your API.


## Why?
How often have you seen APIs like this?
```rust
// BAD EXAMPLE!

fn encrypt(buf: &mut[u8], plaintext: &[u8], key: &[u8], nonce: &[u8])
	-> Result<usize, Box<dyn Error + 'static>>
{
	// Validate parameters
	if plaintext.len() > 65_635 { Err("Plaintext is too large")? }
	if buf.len() < plaintext.len() + 16 { Err("Buffer is smaller than plaintext length")? }
	if key.len() != 32 { Err("Invalid key size")? }
	if nonce.len() != 12 { Err("Invalid nonce size")? }
	
	// Do sth.
	unimplemented!()
}
```
As you can see, this API is pretty opaque and requires a lot of manual checks.

Of course s.o. could use array references:
```rust
// MEH EXAMPLE...

fn encrypt(buf: &mut[u8], plaintext: &[u8], key: &[u8; 32], nonce: &[u8; 12])
	-> Result<usize, Box<dyn Error + 'static>>
{
	// Validate parameters
	if plaintext.len() > 65_635 { Err("Plaintext is too large")? }
	if buf.len() < plaintext.len() + 16 { Err("Buffer is smaller than plaintext length")? }
	
	// Do sth.
	unimplemented!()
}
```
But array references also have their disadvantages. They are not suitable for multiple valid lengths
(allow anything in `16..=32`) nor can they represent relative relationships. Also converting between
other data types and arrays can get annoying.

`len_constraints` tries to solve this problem:
```rust
// GOOD EXAMPLE :D

use std::{ convert::TryInto, error::Error };
use len_constraints::{
	slice_mut::RelativeMut, slice::{ Fixed, Ranged },
	type_math::{ Add, _0, _12, _16, _32, _65536 }
};

fn encrypt(buf: RelativeMut<u8, Add, _16>, plaintext: Ranged<u8, _0, _65536>,
	key: Fixed<u8, _32>, nonce: Fixed<u8, _12>) -> Result<usize, Box<Error + 'static>>
{
	// Get buffer (we do this here because there may not be a relationship at an earlier stage)
	let buf = buf.get_slice_mut(plaintext.len())?;

	// Do sth.
	Ok(7)
}

fn main() -> Result<(), Box<Error + 'static>> {
	// Parameters
	let mut buf: &mut[u8] = &mut[0; 9 + 16];
	let plaintext: &[u8] = b"Testolope";
	let key: &[u8] = &[0; 32];
	let nonce = "12 byte Nonc".as_bytes();

	// Call function
	encrypt(buf.into(), plaintext.try_into()?, key.try_into()?, nonce.try_into()?)?;
	Ok(())
}
```
As you can see, we now can describe complex relationships in the function signature – this makes the
API more transparent and removes the need for manual (and error-prone) parameter validation. Also,
the API is slice-friendly.