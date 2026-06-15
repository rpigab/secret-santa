use thiserror::Error;

/// The generic type of errors emanating from secret-santa-core
#[derive(Error, Debug)]
pub enum Grinch {
    #[error("no solution found")]
    NoSolutionFound,
    #[error("random pick failed")]
    RandomPickFailed,
    #[error("invalid method name: {method_name}")]
    InvalidMethodName { method_name: String },
    #[error("cannot solve graph: {reason}")]
    CannotSolveGraph { reason: String },
    #[error("cannot add edge to graph")]
    CannotAddEdgeToGraph,
    #[error("cannot remove edge from graph")]
    CannotRemoveEdgeFromGraph,
    #[error("cannot find node name in graph by id")]
    CannotFindNodeNameById,
    #[error("cannot find node id in graph by name")]
    CannotFindNodeIdByName,
    #[error("input data is not valid")]
    CouldNotBuildInputData,
}
