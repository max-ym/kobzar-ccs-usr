use super::*;
use std::collections::BTreeSet;

pub type ChannelSet = BTreeSet<Channel>;

pub type ServiceSet = BTreeSet<Service>;

pub type ServiceArchSet<S: ServiceArchitecture> = BTreeSet<S>;

pub type ObjectSet = BTreeSet<Object>;

pub type ObjectArchSet<O: ObjectArchitecture> = BTreeSet<O>;

pub type InterfaceSet = BTreeSet<Interface>;
