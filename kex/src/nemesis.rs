use std::collections::{HashMap, HashSet};
use crate::common::{Message, NodeId, Tick};

pub enum NemesisAction {
    Partition {
        group_a: Vec<NodeId>,
        group_b: Vec<NodeId>,
    },
    HealPartition {
        group_a: Vec<NodeId>,
        group_b: Vec<NodeId>,
    },
    CrashNode(NodeId),
    RestartNode(NodeId),
    SlowLink {
        from: NodeId,
        to: NodeId,
        distribution: LatencyDistribution
    },
    HealLink {
        from: NodeId,
        to: NodeId,
    }
}

pub enum NemesisVerdict {
    Allow(Message),
    Drop,
    Delay(Message, Tick),
}

pub struct Nemesis {
    partitions: Vec<(HashSet<NodeId>, HashSet<NodeId>)>,
    crashed_nodes: HashSet<NodeId>,
    link_overrides: HashMap<(NodeId, NodeId), Tick>
}

impl Nemesis {
    pub fn new() -> Nemesis {
        Self {
            partitions: Vec::new(),
            crashed_nodes: HashSet::new(),
            link_overrides: HashMap::new(),
        }
    }

    pub(crate) fn on_send(&mut self, from: NodeId, to: NodeId, msg: Message, _now: Tick) -> NemesisVerdict {
        todo!()
    }

    //
    fn on_action(&mut self, action: NemesisAction) {
        todo!()
    }
}

struct LatencyDistribution {}