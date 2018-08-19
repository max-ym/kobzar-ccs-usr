use std::cmp::Ordering;

/// Version meta. Usually used in interfaces to mark compatibility
/// of implementers and interface users.
///
/// # Tuples
/// Versions can be build from parts and turned into them when needed.
/// ```
/// # use kobzar_ccs_usr::meta::Version;
/// let parts = (1, 0, 0);
/// let version = Version::from(parts);
/// assert_eq!(Version::new(1, 0, 0), version);
/// ```
///
/// # Comparison
/// When comparing versions, Version with greater major part is
/// greater than that with lesser major version. The same stands
/// for minor version. When both minor and major versions are equal,
/// patch versions get compared. If patches are equal, then these Versions
/// are equal on compare.
///
/// ```
/// # use kobzar_ccs_usr::meta::Version;
/// let tuple = (1, 0, 0);
///
/// // First option of creating Version from tuple.
/// let ver1 = Version::from(tuple);
///
/// // Second option of creating Version from tuple.
/// let ver2: Version = (1, 1, 0).into();
///
/// assert!(ver1 < ver2);
/// ```
///
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Version {

    major   : usize,
    minor   : usize,
    patch   : usize,
}


impl Version {

    /// Create new version from given parts.
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Version { major, minor, patch }
    }

    /// Major version number.
    pub fn major(&self) -> usize {
        self.major
    }

    /// Minor version number.
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Patch version number.
    pub fn patch(&self) -> usize {
        self.patch
    }
}

impl Default for Version {

    fn default() -> Version {
        Version::new(0, 0, 0)
    }
}

impl From<(usize, usize, usize)> for Version {

    fn from(parts: (usize, usize, usize)) -> Version {
        Version::new(parts.0, parts.1, parts.2)
    }
}

impl Into<(usize, usize, usize)> for Version {

    fn into(self) -> (usize, usize, usize) {
        (self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {

    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {

    fn cmp(&self, other: &Version) -> Ordering {
        use self::Ordering::*;

        let major = self.major.cmp(&other.major);
        match major {
            Greater => Greater,
            Less => Less,
            Equal => {
                let minor = self.minor.cmp(&other.minor);
                match minor {
                    Greater => Greater,
                    Less => Less,
                    Equal => {
                        let patch = self.patch.cmp(&other.patch);
                        match patch {
                            Greater => Greater,
                            Less => Less,
                            Equal => Equal,
                        }
                    }
                }
            }
        }
    }
}
