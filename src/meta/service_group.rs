use super::*;

/// Service group by visibility. Used by objects to make code simpler
/// by grouping all service types into single field.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceGroup<S: ServiceArchitecture> {

    pub public: ServiceArchSet<S>,

    pub private: ServiceArchSet<S>,

    pub internal: ServiceArchSet<S>,
}

impl<T> Default for ServiceGroup<T>
        where T: ServiceArchitecture {

    fn default() -> ServiceGroup<T> {
        ServiceGroup {
            public: ServiceArchSet::default(),
            private: ServiceArchSet::default(),
            internal: ServiceArchSet::default(),
        }
    }
}
