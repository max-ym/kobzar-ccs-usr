use super::*;
use std::collections::BTreeSet;

pub type ChannelSet = BTreeSet<Channel>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelArchSet<C: ChannelArchitecture>(BTreeSet<C>);

pub type ServiceSet = BTreeSet<Service>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceArchSet<S: ServiceArchitecture>(BTreeSet<S>);

pub type ObjectSet = BTreeSet<Object>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectArchSet<O: ObjectArchitecture>(BTreeSet<O>);

pub type InterfaceSet = BTreeSet<Interface>;

impl<T> Default for ServiceArchSet<T>
        where T: ServiceArchitecture {

    fn default() -> Self {
        ServiceArchSet(BTreeSet::new())
    }
}

impl<T> Default for ChannelArchSet<T>
        where T: ChannelArchitecture {

    fn default() -> Self {
        ChannelArchSet(BTreeSet::new())
    }
}

impl<T> Default for ObjectArchSet<T>
        where T: ObjectArchitecture {

    fn default() -> Self {
        ObjectArchSet(BTreeSet::new())
    }
}
