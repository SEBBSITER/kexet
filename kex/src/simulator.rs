use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use event::ScheduledEvent;
use crate::common::NodeId;
use crate::event;
use crate::event::Event;
use crate::network::Network;
use crate::node::Node;

// Discrete-event simulation with a logical clock
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub u64);

pub struct Simulator {
    clock: Tick,
    seq_counter: u64,
    queue: BinaryHeap<Reverse<ScheduledEvent>>,
    nodes: HashMap<NodeId, Box<dyn Node>>,
    network: Network,
}

impl Simulator {
    pub fn next_seq(&mut self) -> u64 {
        let seq = self.seq_counter;
        self.seq_counter += 1;
        seq
    }

    fn enqueue(&mut self, time: Tick, event: Event) {
        let seq = self.next_seq();
        self.queue.push(Reverse(ScheduledEvent::new(time, seq, event)));
    }

    pub fn run(&mut self) {
        while let Some(Reverse(event)) = self.queue.pop() {
            // Time must be monotonic
            let event_time = event.get_time();
            debug_assert!(event_time >= self.clock, "time must be monotonic");
            self.clock = event_time;

            // TODO: Complete
            let action = match event.get_event() {
                Event::SendMessage { from, to, payload } => {}
                Event::DeliverMessage { from, to, payload } => {}
                Event::BeginTraining { node, round } => {},
                Event::Deliver { .. } => {}
            };
        }
    }
}