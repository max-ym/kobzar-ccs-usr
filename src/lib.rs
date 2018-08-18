/// Main module of the CCS user library. Traits presented in this module
/// are implementing handles for objects, channels and services and not
/// direct information.
pub mod ccs {

    /// Object have local network and can own services. This is a handle
    /// that allows accessing object allocated in the network.
    pub trait Object {

        /// The name of this object.
        fn name(&self) -> &str;

        /// ID assigned to the object at current network level.
        /// Used to uniquely identify the objects. Even if some
        /// of them have the same name.
        fn id(&self) -> usize;

        /// Network this object is located in if it is known.
        fn parent_network(&self) -> Option<Network>;

        /// Local network of this object.
        fn local_network(&self) -> Network;

        /// Object where current thread is located.
        fn myself() -> Self;
    }

    /// Service of some object. All service structs derive this trait.
    pub trait Service {

        /// The name of this service.
        fn name(&self) -> &str;

        /// Object where the service is located.
        fn object(&self) -> Object;

        /// Service where current thread started.
        fn myself() -> Self;
    }

    /// Channel established to transmit information among services.
    pub trait Channel {

        /// The services connected to the channel.
        fn connected_services(&self) -> Iterator<Item = &Service>;

        /// Create new channel with this service involved.
        fn new() -> Option<Self>;
    }

    pub trait Network {

        /// Name of the network.
        fn name(&self) -> &str;

        /// Iterator over objects that can be accessed by active service.
        /// Objects are searched by their names.
        fn object_by_name(name: &str) -> Iterator<Item = &Object>;
    }

    /// Interface with some set of functions to be implemented.
    pub trait Interface {

        /// Full path and name of the interface.
        fn path(&self) -> Path;

        /// Interface version.
        fn version(&self) -> Version;

        /// Services required to be implemented in order to support
        /// this interface.
        fn services(&self) -> Iterator<Item = ServiceMeta>;

        /// Dependency on other interfaces that must be implemented
        /// in order this interface could be supported by the implementing
        /// object.
        fn dependencies(&self) -> Iterator<Item = Interface>;
    }
}
