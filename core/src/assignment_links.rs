use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct AssignmentLink {
    pub giver_name: String,
    /// relative uri containing only query params
    pub recipient_link: String,
}

#[derive(Debug)]
pub struct AssignmentLinks {
    pub(crate) assignments_links: HashSet<AssignmentLink>,
}

impl AssignmentLinks {
    pub fn assignments_links(&self) -> &HashSet<AssignmentLink> {
        &self.assignments_links
    }
}
