use super::*;

/// Channel that allows communication among services.
#[derive(Clone)]
pub struct Channel {

    /// ID of the channel registered in the system.
    id: usize,

    /// The service which originated this channel. Even though service
    /// could give up this channel, it still is identified as that which
    /// has origin from that service. The ID of the channel is related
    /// to the local network of the object of origin service.
    origin: Service,
}

/// Error which appers when using Channel.
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
}

type ChannelResult<T> = Result<T, ChannelError>;

impl Channel {

    /// ID that uniquely identifies this channel inside the object.
    pub fn id(&self) -> usize {
        self.id
    }

    /// See [origin service](struct.Channel.html#structfield.origin).
    pub fn origin(&self) -> &Service {
        &self.origin
    }

    /// Leave this channel. Service which called this fn leaves the channel
    /// but still has a handle to it and can retrieve information
    /// about the channel. If channel policy allows, Service can
    /// join the channel again.
    pub fn leave(&self) -> ChannelResult<()> {
        unimplemented!()
    }

    /// Invite some service to join this channel.
    pub fn invite(&self, service: &Service) -> ChannelResult<()> {
        unimplemented!()
    }

    /// Try joining the channel. If this service already is a member then
    /// error occurs but service still will remain the member.
    /// If channel policy forbids joining without invitation then
    /// this service won't join and receives an error.
    pub fn join(&self) -> ChannelResult<()> {
        unimplemented!()
    }

    /// How many threads are connected to the channel.
    pub fn connections_count(&self) -> usize {
        unimplemented!()
    }

    /// Channel current policy.
    pub fn policy(&self) -> meta::Policy {
        unimplemented!()
    }
}
