#[cfg(all(feature = "client", feature = "http2"))]
mod compat;
mod rewind;

#[cfg(all(feature = "client", feature = "http2"))]
pub(crate) use self::compat::Compat;
pub(crate) use self::rewind::Rewind;
