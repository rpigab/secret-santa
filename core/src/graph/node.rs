pub type NodeId = u16;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Node {
    id: NodeId,
    name: String,
}

impl Node {
    pub(crate) fn new(id: NodeId, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    //

    pub(crate) fn id(&self) -> &NodeId {
        &self.id
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }
}