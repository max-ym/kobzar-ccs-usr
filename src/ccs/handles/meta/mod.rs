//! Metadata parts of network elements.

/// Implementation and declaration for Version struct.
mod version;
pub use self::version::*;

/// Declaration and implementation for Path struct.
mod path;
pub use self::path::*;

/// Policy of channel.
mod chan_policy;
pub use self::chan_policy::*;
