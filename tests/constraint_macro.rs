#[macro_use] extern crate len_constraints;
use len_constraints::{
	slice_mut::RelativeMut, slice::{ Fixed, Ranged },
	type_math::{ Add, _0, _12, _16, _32, _65536 }
};
use std::error::Error;


#[test]
fn test() -> Result<(), Box<Error + 'static>> {
	let buf: &mut[u8] = &mut[0; 16];
	let plaintext: &[u8] = &[];
	let key: &[u8] = &[0; 32];
	let nonce: &[u8] = &[0; 12];
	
	
	constraints! {
		buf => RelativeMut<u8, Add, _16> [plaintext.len()],
		plaintext => Ranged<u8, _0, _65536>,
		key => Fixed<u8, _32>,
		nonce => Fixed<u8, _12>
	};
	
	
	assert_eq!(
		format!("buf: {:?}", buf),
		"buf: RelativeMut { slice: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], constraint: PhantomData }"
	);
	assert_eq!(
		format!("plaintext: {:?}", plaintext),
		"plaintext: Ranged { slice: [], constraint: PhantomData }"
	);
	assert_eq!(
		format!("key: {:?}", key),
		"key: Fixed { slice: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], constraint: PhantomData }"
	);
	assert_eq!(
		format!("nonce: {:?}", nonce),
		"nonce: Fixed { slice: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], constraint: PhantomData }"
	);
	Ok(())
}
