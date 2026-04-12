use crate::common::{Message, NodeId, Tick};

pub enum Input<'a> {
    Message { from: NodeId, msg: &'a Message },
    Timeout { tag: u64 }
}

pub enum Action {
    Send { to: NodeId, msg: Message },
    Broadcast { message: Message },
    SetTimeout { delay: Tick, tag: u64 },
    CancelTimeout { tag: u64 },
    // ReportMetric(MetricEvent),
}

pub trait Node {
    fn on_event(&mut self, input: Input, now: Tick) -> Vec<Action>;
    fn id(&self) -> NodeId;
}