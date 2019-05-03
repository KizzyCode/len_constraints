/// Convert multiple variables in-place into their constrained representation
///
/// ## Example
/// ```ignored
/// constraints! {
/// 	buf => RelativeMut<u8, Add, _16> [plaintext.len()],
/// 	plaintext => Ranged<u8, _0, _65536>,
/// 	key => Fixed<u8, _32>,
/// 	nonce => Fixed<u8, _12>
/// };
///
/// // Print the types
/// println!("{:?}", buf);       // RelativeMut { slice: [ ... ], constraint: PhantomData }
/// println!("{:?}", plaintext); // Ranged { slice: [ ... ], constraint: PhantomData }
/// println!("{:?}", key);       // Fixed { slice: [ ... ], constraint: PhantomData }
/// println!("{:?}", nonce);     // Fixed { slice: [ ... ], constraint: PhantomData }
/// ```
#[macro_export]
macro_rules! constraints {
	($source:ident => $constrained:ty) => {
		let $source: $constrained = ::std::convert::TryFrom::try_from($source)?;
	};
	($source:ident => $constrained:ty [$len:expr]) => {
		let $source = <$constrained>::try_from($source, $len)?;
	};
	($( $source:ident => $constrained:ty $([$len:expr])* ),+) => {
		$( constraints!($source => $constrained $([$len])*) );+
	}
}