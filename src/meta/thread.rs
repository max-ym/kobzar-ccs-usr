use super::*;
use std::rc::Rc;

/// Thread state.
#[derive(Clone, Copy, Debug)]
pub enum ThreadState {

    /// Just running as usual.
    Active,

    /// Waiting for channel event.
    Wait,

    /// Thread finished execution and is dead.
    Dead,
}

/// Architecture-dependent implementation of memory.
pub trait MemoryArchitecture {
}

/// Architecture-dependent implementation of Thread.
pub trait ThreadArchitecture {

    type MA: MemoryArchitecture;

    type C: ChannelArchitecture;

    /// Architecture-independent part of Thread.
    fn unarch(&self) -> &Thread;

    /// Memory of this thread.
    fn memory(&self) -> &Self::MA;

    /// Current [State](enum.ThreadState.html) of this thread.
    fn state(&self) -> ThreadState;

    /// Set of all connected channels.
    fn connected_channels(&self) -> ChannelArchSet<Self::C>;
}

/// Thread that is performing in the system.
#[derive(Clone)]
pub struct Thread {

    /// Unique ID of this thread in some Object.
    id: usize,

    /// Origin service instance of which this thread is.
    origin: Rc<Service>,
}

impl Thread {

    /// Unique ID of this thread in some Object.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Origin service instance of which this thread is.
    pub fn origin(&self) -> &Service {
        &self.origin
    }
}
