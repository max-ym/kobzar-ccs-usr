/// Channel policy that defines the operations handles can apply.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Policy {

    /// Whether not-a-members of the channel can invite Services to join.
    invitation_from_not_member: bool,

    /// Whether to allow origin service to leave the channel when there
    /// still is active threads connected.
    origin_can_leave: bool,

    /// Whether to allow to join by handle without invitation.
    join_by_handle: bool,
}

/// Struct that simplifies building the policy.
#[derive(Clone, Copy, Default, Debug)]
pub struct PolicyBuilder {
    invitation_from_not_member: Option<bool>,
    origin_can_leave: Option<bool>,
    join_by_handle: Option<bool>,
}

impl PolicyBuilder {

    /// Create new policy builder.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn invitation_from_not_member(&mut self, val: bool) {
        self.invitation_from_not_member = Some(val);
    }

    pub fn origin_can_leave(&mut self, val: bool) {
        self.origin_can_leave = Some(val);
    }

    pub fn join_by_handle(&mut self, val: bool) {
        self.join_by_handle = Some(val);
    }

    /// Try building a Policy instance. If any of the fields
    /// were unassigned then this builder will fail and return self
    /// in error message. If everything goes well then policy instance
    /// will be returned instead.
    pub fn build(self) -> Result<Policy, Self> {
        let invitation_from_not_member
                = if let Some(v) = self.invitation_from_not_member {
            v
        } else {
            return Err(self);
        };

        let origin_can_leave = if let Some(v) = self.origin_can_leave {
            v
        } else {
            return Err(self);
        };

        let join_by_handle = if let Some(v) = self.join_by_handle {
            v
        } else {
            return Err(self);
        };

        Ok(Policy {
            invitation_from_not_member,
            origin_can_leave,
            join_by_handle,
        })
    }
}
