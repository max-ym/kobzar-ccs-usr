use super::*;
use std::rc::Rc;

/// Iterator over Object handles.
#[derive(Clone)]
pub struct ObjectIterator {

    /// Vector containing all Objects.
    vec: Vec<Object>,

    /// Current iterator index.
    i: usize,
}

/// Iterator over Service handles.
#[derive(Clone)]
pub struct ServiceIterator {

    /// Vector containing all Services.
    vec: Vec<Service>,

    /// Current iterator index.
    i: usize,
}

/// Iterator over Channel handles.
pub struct ChannelIterator {

    /// Vector containing all Channels.
    vec: Vec<Channel>,

    /// Current iterator index.
    i: usize,
}

/// Iterator over [path nodes](some.html).
pub struct PathNodeIterator {

    /// Last node of the path.
    last: Rc<Path>,
}

impl Iterator for ObjectIterator {

    type Item = Object;

    fn next(&mut self) -> Option<Object> {
        if self.i >= self.vec.len() {
            // Iterator is exhausted.
            return None;
        }

        let obj = self.vec[self.i].clone();
        self.i += 1;
        Some(obj)
    }
}

impl ExactSizeIterator for ObjectIterator {

    fn len(&self) -> usize {
        self.vec.len()
    }
}

impl Iterator for ServiceIterator {

    type Item = Service;

    fn next(&mut self) -> Option<Service> {
        if self.i >= self.vec.len() {
            // Iterator is exhausted.
            return None;
        }

        let serv = self.vec[self.i].clone();
        self.i += 1;
        Some(serv)
    }
}

impl ExactSizeIterator for ServiceIterator {

    fn len(&self) -> usize {
        self.vec.len()
    }
}

impl Iterator for ChannelIterator {

    type Item = Channel;

    fn next(&mut self) -> Option<Channel> {
        if self.i >= self.vec.len() {
            // Iterator is exhausted.
            return None;
        }

        let chan = self.vec[self.i].clone();
        self.i += 1;
        Some(chan)
    }
}

impl ExactSizeIterator for ChannelIterator {

    fn len(&self) -> usize {
        self.vec.len()
    }
}

impl PathNodeIterator {

    pub fn new(node: Rc<Path>) -> Self {
        PathNodeIterator {
            last: node,
        }
    }
}

impl Iterator for PathNodeIterator {

    type Item = Rc<Path>;

    fn next(&mut self) -> Option<Rc<Path>> {
        // Get Rc of current node.
        let rc = self.last.clone();

        // Move to previous node.
        self.last = self.last.parent().clone();

        Some(rc)
    }
}
