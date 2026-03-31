use crate::common::{Message, NodeId};
use crate::simulator::Tick;

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

pub trait Nemesis {
    fn on_send(&mut self, from: NodeId, to: NodeId, msg: Message, now: Tick) -> NemesisVerdict;

    //
    fn on_action(&mut self, action: NemesisAction);
}

struct LatencyDistribution {}