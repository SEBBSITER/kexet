use crate::common::{EventSeq, Message, NodeId, Tick};
use std::cmp::Ordering;

pub enum Event {
    SendMessage {
        from: NodeId,
        to: NodeId,
        payload: Payload,
    },
    DeliverMessage {
        from: NodeId,
        to: NodeId,
        payload: Payload,
    },
    BeginTraining {
        node: NodeId,
        round: u32,
    },
    Deliver { from: NodeId, to: NodeId, message: Message },
}

// TODO: Maybe could find a better solution than using a separate EventSeq

pub struct ScheduledEvent {
    time: Tick,
    seq: EventSeq, // Monotonic, for deterministic tie-breaking
    event: Event,
}

impl ScheduledEvent {
    pub fn new(time: Tick, seq: EventSeq, event: Event) -> Self {
        Self { time, seq, event }
    }
    pub fn get_time(&self) -> Tick { self.time }
    pub fn get_event(&self) -> &Event { &self.event }
}

// Custom implementations of ordering to make sure intended ordering logic time -> seq is preserved
impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
            .then_with(|| self.seq.cmp(&other.seq))
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.seq == other.seq
    }
}

impl Eq for ScheduledEvent {}

pub struct Payload {}