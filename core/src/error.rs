/// The generic type of errors emanating from secret-santa-core
#[derive(Debug)]
pub enum Grinch {
    NoSolutionFound,
    RandomPickFailed,
    InvalidMethodName { method_name: String },
    CannotSolveGraph { reason: String },
    CannotAddEdgeToGraph,
    CannotRemoveEdgeFromGraph,
    CannotFindNodeNameById,
    CannotFindNodeIdByName,
    CouldNotBuildInputData,
}

impl std::fmt::Display for Grinch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grinch::NoSolutionFound => write!(f, "no solution found"),
            Grinch::RandomPickFailed => write!(f, "random pick failed"),
            Grinch::InvalidMethodName { method_name } => write!(f, "invalid method name: {method_name}"),
            Grinch::CannotSolveGraph { reason } => write!(f, "cannot solve graph: {reason}"),
            Grinch::CannotAddEdgeToGraph => write!(f, "cannot add edge to graph"),
            Grinch::CannotRemoveEdgeFromGraph => write!(f, "cannot remove edge from graph"),
            Grinch::CannotFindNodeNameById => write!(f, "cannont find node name in graph by id"),
            Grinch::CannotFindNodeIdByName => write!(f, "cannot find node id in graph by name"),
            Grinch::CouldNotBuildInputData => write!(f, "input data is not valid")
        }
    }
}
