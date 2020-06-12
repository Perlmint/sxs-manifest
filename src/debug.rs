use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum PathNode {
    Named(&'static str),
    Indexed(usize),
}

impl From<&'static str> for PathNode {
    fn from(val: &'static str) -> Self {
        PathNode::Named(val)
    }
}

impl From<usize> for PathNode {
    fn from(val: usize) -> Self {
        PathNode::Indexed(val)
    }
}

impl Display for PathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathNode::Named(name) => write!(f, "{}", *name),
            PathNode::Indexed(idx) => write!(f, "{}", *idx),
        }
    }
}

#[derive(Debug)]
pub struct Path<'a>(Option<&'a Path<'a>>, PathNode);

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ancestors) = &self.0 {
            write!(f, "{}.", ancestors)?;
        }
        write!(f, ".{}", &self.1)
    }
}

impl<'a> Path<'a> {
    pub fn new(node: PathNode) -> Path<'a> {
        Path(None, node)
    }

    pub fn appended<'b: 'a>(&'b self, node: PathNode) -> Path<'b> {
        Path(Some(&self), node)
    }
}
