pub use self::immutable::Ref;
pub use self::iter::Iter;
pub use self::mutable::Ref as Mut;
pub use self::owned::Owned;

mod immutable;

mod mutable;

mod owned;

mod iter;

#[macro_export]
macro_rules! dict {
	( $($key:expr => $value:expr),* $(,)*) => ({
			let mut dict = ::ffmpeg::Dictionary::new();

			$(
				dict.set($key, $value);
			)*

			dict
		}
	);
}
