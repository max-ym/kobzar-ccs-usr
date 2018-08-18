use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};
use std::borrow::Borrow;

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

/// Path to some interface or object.
pub struct Path {

    /// Name of current path node.
    name: Name,

    /// Parent node if any. Root node never has a parent. All
    /// other nodes are initialized with some parent node.
    parent: Rc<Path>,

    /// Self weak reference. Used to pass self reference to children.
    selfref: Weak<Path>,

    /// Map of children.
    children: BTreeMap<WeakName, Rc<Path>>,
}

impl Name {

    /// Create new Name from string. The Name will not be created if
    /// it violates the name rules. For more details about the
    /// rules see [Name](struct.Name.html).
    pub fn try_new(s: &str) -> Option<Name> {
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

    pub fn weak(&self) -> WeakName {
        WeakName(self as *const _)
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

impl Path {

    /// Try creating new Path with given name. This node is treated
    /// as path's parent. The passed name is validated and in case
    /// it is invalid None will be returned. Otherwise, the path is
    /// returned and node is registered as child in current one.
    pub fn try_new(&mut self, name: &str) -> Option<Rc<Path>> {
        use std::str::FromStr;

        // Validate the name.
        let name = Name::try_new(name);
        if name.is_none() {
            // Name is invalid.
            return None;
        }
        let name = name.unwrap();

        let path = Path {
            name: name,
            parent: self.selfref.upgrade().unwrap(),

            // Currently nothing to assign. Will have
            // valid value soon.
            selfref: Weak::new(),

            children: Default::default(),
        };

        // Wrap new path node into Rc.
        let rc = Rc::new(path);

        // Save this node as current node's child.
        self.children.insert(rc.name.weak(), rc.clone());

        // Tell new node it's reference.
        self.selfref = Rc::downgrade(&rc);

        Some(rc)
    }

    /// This node name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
