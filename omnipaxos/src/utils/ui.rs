use crate::{
    ballot_leader_election::Ballot,
    messages::ballot_leader_election::HeartbeatReply,
    storage::Entry,
    util::{LeaderState, NodeId},
    HashMap,
};

/// The states of all the nodes in the cluster.
#[derive(Debug, Clone, Default)]
pub struct ClusterState {
    /// The accepted indexes of all the nodes in the cluster. The index of the vector is the node id.
    pub accepted_indexes: HashMap<NodeId, usize>,
    /// All the received heartbeats from the previous heartbeat round, including the current node.
    /// Represents nodes that are currently alive from the view of the current node.
    pub heartbeats: Vec<HeartbeatReply>,
}

impl<T> From<&LeaderState<T>> for ClusterState
where
    T: Entry,
{
    fn from(leader_state: &LeaderState<T>) -> Self {
        let accepted_indexes = leader_state.accepted_indexes.clone();
        Self {
            accepted_indexes,
            heartbeats: vec![],
        }
    }
}

/// The states that are for UI to show.
pub struct OmniPaxosStates {
    /// The current ballot
    pub current_ballot: Ballot,
    /// The current leader
    pub current_leader: Option<NodeId>,
    /// The current decided index
    pub decided_idx: usize,
    /// All the received heartbeats from the previous heartbeat round, including the current node.
    /// Represents nodes that are currently alive from the view of the current node.
    pub heartbeats: Vec<HeartbeatReply>,
    /// The states of all the nodes in the cluster.
    pub cluster_state: ClusterState,
}
