//! The local application implementation of CCS.
//! Used as default in-app library when global network is not available
//! or required.
//!
//! This is useful to test different aspects of CCS and was designed for
//! this purpose.
//!
//! This module can be used as tutorial for implementing your own CCS
//! master.

use super::meta::{Interface, InterfaceSet};

/// The implementer of Interface source. When application begins,
/// we add the list of all interfaces we will use so that master could
/// find all requested sources when they get needed.
pub struct MyInterfaceSource {

    /// The set of application interfaces.
    ints: InterfaceSet,
}

impl MyInterfaceSource {

    /// Append new interface.
    pub fn append(&mut self, i: Interface) {
        self.ints.insert(i);
    }
}
