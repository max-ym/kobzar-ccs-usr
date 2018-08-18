use super::*;
use std::collections::BTreeSet;

pub type ChannelSet = BTreeSet<Channel>;

pub type ServiceSet = BTreeSet<Service>;

pub type ObjectSet = BTreeSet<Object>;

pub type InterfaceSet = BTreeSet<Interface>;

pub type ObjectArchSet<T: ObjectArchitecture> = BTreeSet<T>;
