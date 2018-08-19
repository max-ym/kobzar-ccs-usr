use super::*;
use std::collections::BTreeSet;

pub type ChannelSet = BTreeSet<Channel>;

pub struct ChannelArchSet<C: ChannelArchitecture>(BTreeSet<C>);

pub type ServiceSet = BTreeSet<Service>;

pub struct ServiceArchSet<S: ServiceArchitecture>(BTreeSet<S>);

pub type ObjectSet = BTreeSet<Object>;

pub struct ObjectArchSet<O: ObjectArchitecture>(BTreeSet<O>);

pub type InterfaceSet = BTreeSet<Interface>;
