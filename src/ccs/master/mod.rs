//! Master module that allows allocating new channels, services and objects.

use std::rc::Rc;

use super::*;
use super::meta;

mod interface_source;
pub use self::interface_source::*;

mod implementer_source;
pub use self::implementer_source::*;
