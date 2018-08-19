//! Useful tools that implement common algorithms and are used in
//! the architecture-dependent tasks.

/// Analizes dependencies of threads, channels between them and
/// state of threads and channels.
///
/// Find cycled dependencies - when all threads are waiting for
/// a signal which cannot appear because each thread is waiting forever.
/// It's halted state must be detected and eliminated by updating
/// channel state and notifying services about the halted state.
mod channel_resolver;
pub use self::channel_resolver::*;


