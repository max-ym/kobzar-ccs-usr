use std::collections::BTreeMap;
use std::rc::Rc;
use std::convert::AsRef;
use std::cmp::Ordering;

/// The name of the path node.
/// It stores only the valid name of the node. Before assigning
/// the value it checks whether assigned name is valid.
///
/// Valid are any latin alphabetic characters, numbers and underscores.
/// Name cannot begin with a number. Name cannot be empty.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Name(String);

/// The name of the path node. Used in maps (like
/// std::collections::BTreeMap) to order the elements but avoid clonning
/// actual names. The instance is expected to live less than the data
/// it refers to. Usually it is achieved by deleting map entry
/// first and then releasing the child it pointed to.
#[derive(Clone, Copy)]
struct WeakName(*const Name);

///
pub struct Path {

    /// The name of this path.
    name: Name,

    /// Reference to parent path node.
    parent: Rc<Path>,

    /// Map of all children of this path node.
    children: BTreeMap<WeakName, Path>,
}

impl Name {

    /// Create new Name from string. The Name will not be created if
    /// it violates the name rules. For more details about the
    /// rules see [Name](struct.Name.html).
    pub fn new(s: &str) -> Option<Name> {
        if s.is_empty() {
            // Name cannot be empty.
            return None;
        }

        let mut chars = s.chars();
        let first = chars.next().unwrap();

        if !Self::is_allowed_first_char(first) {
            // First characted is invalid.
            return None;
        }

        // Expect all remaining characters.
        for c in chars {
            if !Self::is_allowed_char(c) {
                // Current character is invalid.
                return None;
            }
        }

        use std::str::FromStr;
        Some(Name(String::from_str(s).unwrap()))
    }

    /// Whether passed character can be used in the name.
    fn is_allowed_char(c: char) -> bool {
        Self::is_allowed_first_char(c)
        ||
        (c >= '0' && c <= '9')
    }

    /// Whether passed character can be used in the beginning of path node
    /// name.
    fn is_allowed_first_char(c: char) -> bool {
        (c >= 'A' && c <= 'Z')
        ||
        (c >= 'a' && c <= 'z')
        ||
        c == '_'
    }
}

impl AsRef<str> for Name {

    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl WeakName {

    /// Create new WeakName reference to given name.
    ///
    /// # Safety
    /// Name must outlive WeakName instance. Otherwise, the struct will
    /// refer to unexisting data.
    pub unsafe fn from(name: &Name) -> Self {
        WeakName(name as  _)
    }
}

impl AsRef<Name> for WeakName {

    fn as_ref(&self) -> &Name {
        unsafe { &*self.0 }
    }
}

impl AsRef<str> for WeakName {

    fn as_ref(&self) -> &str {
        unsafe { (&*self.0).as_ref() }
    }
}

impl PartialEq for WeakName {

    fn eq(&self, other: &WeakName) -> bool {
        let s0: &str = self.as_ref();
        let s1: &str = self.as_ref();

        s0 == s1
    }
}

impl Eq for WeakName {}

impl PartialOrd for WeakName {

    fn partial_cmp(&self, other: &WeakName) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeakName {

    fn cmp(&self, other: &WeakName) -> Ordering {
        let s0: &str = self.as_ref();
        let s1: &str = self.as_ref();

        s0.cmp(s1)
    }
}
