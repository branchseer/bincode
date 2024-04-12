#[cfg(feature = "alloc")]
mod impl_alloc;
#[cfg(feature = "alloc")]
pub use self::impl_alloc::*;

#[cfg(feature = "std")]
mod impl_std;
#[cfg(feature = "std")]
pub use self::impl_std::*;

#[cfg(feature = "derive")]
mod derive;
#[cfg(feature = "derive")]
pub use self::derive::*;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;

#[cfg(feature = "index_vec")]
mod index_vec;

#[cfg(feature = "indexmap")]
mod indexmap;

#[cfg(feature = "compact_str")]
mod compact_str;
