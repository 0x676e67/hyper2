#[cfg(all(feature = "client", feature = "http1"))]
pub(crate) mod buf;
pub(crate) mod io;
#[cfg(all(feature = "client", feature = "http1"))]
pub(crate) mod task;
#[cfg(all(feature = "client", feature = "http2"))]
pub(crate) mod time;
#[cfg(all(feature = "client", feature = "http1"))]
pub(crate) mod watch;
