/// Main module of the CCS user library. Traits presented in this module
/// are implementing handles for objects, channels and services and not
/// direct information.
pub mod ccs {

    pub mod meta;
    pub use self::meta::*;

    pub mod master;

    pub mod local;
}
