use super::*;
use std::rc::Rc;

/// Channel that allows communication among services.
#[derive(Clone)]
pub struct Channel {

    /// ID of the channel registered in the system.
    id: usize,

    /// The service which originated this channel. Even though service
    /// could give up this channel, it still is identified as that which
    /// has origin from that service. The ID of the channel is related
    /// to the local network of the object of origin service.
    origin: Rc<Service>,
}

/// Error which appears when using Channel.
pub enum ChannelError {

    /// This service is no longer a member
    /// of the channel or maybe never was.
    NotMember,

    /// Cannot join some channel because the service already is
    /// a member.
    AlreadyMember,

    /// Cannot join the channel because this operation is
    /// forbiden for current service.
    JoinForbiden,

    /// Multiple connections to the channel are forbiden. Only
    /// peer-to-peer single connections are allowed.
    MultipleConnectionsForbiden,
}

pub type ChannelResult<T> = Result<T, ChannelError>;

/// Architecture-dependent part of Channel.
pub trait ChannelArchitecture {

    type S: ServiceArchitecture;

    /// Leave this channel. Service which called this fn leaves the channel
    /// but still has a handle to it and can retrieve information
    /// about the channel. If channel policy allows, Service can
    /// join the channel again.
    fn leave(&self) -> ChannelResult<()>;

    /// Invite some service to join this channel.
    fn invite(&self, service: &Self::S) -> ChannelResult<()>;

    /// Try joining the channel. If this service already is a member then
    /// error occurs but service still will remain the member.
    /// If channel policy forbids joining without invitation then
    /// this service won't join and receives an error.
    fn join(&self) -> ChannelResult<()>;

    /// How many threads are connected to the channel.
    fn connections_count(&self) -> usize;

    /// Channel current policy.
    fn policy(&self) -> Policy;

    /// The handle for this channel.
    fn handle(&self) -> &Channel;
}

impl Channel {

    /// ID that uniquely identifies this channel inside the object.
    pub fn id(&self) -> usize {
        self.id
    }

    /// See [origin service](struct.Channel.html#structfield.origin).
    pub fn origin(&self) -> &Service {
        &self.origin
    }
}
