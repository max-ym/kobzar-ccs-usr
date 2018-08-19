macro_rules! unwrap {
    ($self: ident, $name: ident) => {
        let $name = if let Some(v) = $self.$name {
            v
        } else {
            return Err($self);
        };
    }
}

/// Channel policy that defines the allowed operations over channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Policy {

    /// Whether not-a-members of the channel can invite Services to join.
    invitation_from_not_member: bool,

    /// Whether to allow origin service to leave the channel when there
    /// still is active threads connected.
    origin_can_leave: bool,

    /// Whether to allow to join by handle without invitation.
    join_by_handle: bool,

    /// Whether allowed to have multiple thread connected to this channel.
    /// If forbider, only single peer-to-peer is allowed.
    no_multiple_connectons: bool,
}

/// Struct that simplifies building the policy.
///
/// First you need to create new policy builder. Then you need to assign
/// each of the fields by calling same name methods. When you finished
/// setting the values you can call [build](#method.build) and acquire
/// ready to use policy.
///
/// # Example
/// ```
/// # use kobzar_ccs_usr::meta::PolicyBuilder;
/// let mut builder = PolicyBuilder::new();
///
/// builder.invitation_from_not_member(true);
/// builder.origin_can_leave(true);
/// builder.join_by_handle(false);
/// builder.no_multiple_connectons(false);
///
/// let result = builder.build();
///
/// assert!(result.is_ok());
/// ```
///
/// When there are still unassigned fields builder will fail to
/// build the policy and instead it will return Err result and
/// pass itself as Err field.
/// ```
/// # use kobzar_ccs_usr::meta::PolicyBuilder;
/// let mut builder = PolicyBuilder::new();
///
/// builder.invitation_from_not_member(true);
/// builder.origin_can_leave(true);
/// // Some fields are not set.
/// let result = builder.build();
///
/// assert!(result.is_err());
///
/// // Still can use builder to assign missing values.
/// let mut builder = result.unwrap_err();
/// builder.join_by_handle(false);
/// builder.no_multiple_connectons(false);
///
/// // Now it works.
/// let result = builder.build();
/// assert!(result.is_ok());
/// ```
#[derive(Clone, Copy, Default, Debug)]
pub struct PolicyBuilder {
    invitation_from_not_member: Option<bool>,
    origin_can_leave: Option<bool>,
    join_by_handle: Option<bool>,
    no_multiple_connectons: Option<bool>,
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

    pub fn no_multiple_connectons(&mut self, val: bool) {
        self.no_multiple_connectons = Some(val);
    }

    /// Try building a Policy instance. If any of the fields
    /// were unassigned then this builder will fail and return self
    /// in error message. If everything goes well then policy instance
    /// will be returned instead.
    pub fn build(self) -> Result<Policy, Self> {
        unwrap!(self, invitation_from_not_member);
        unwrap!(self, origin_can_leave);
        unwrap!(self, join_by_handle);
        unwrap!(self, no_multiple_connectons);

        Ok(Policy {
            invitation_from_not_member,
            origin_can_leave,
            join_by_handle,
            no_multiple_connectons,
        })
    }
}
