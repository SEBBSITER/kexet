use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use event::ScheduledEvent;
use crate::common::{NodeId, Tick};
use crate::event;
use crate::event::Event;
use crate::network::Network;
use crate::node::Node;

pub struct Simulator {
    clock: Tick,
    seq_counter: u64,
    queue: BinaryHeap<Reverse<ScheduledEvent>>,
    network: Network,
}

impl Simulator {
    pub fn new(clock: Tick, seq_counter: u64, network: Network) -> Simulator {
        Self {
            clock,
            seq_counter,
            queue: BinaryHeap::new(),
            network,
        }
    }

    pub fn next_event(&mut self) -> Option<ScheduledEvent> {
        if let Some(Reverse(event)) = self.queue.pop() {
            Some(event)
        } else {
            None
        }
    }

    pub fn next_seq(&mut self) -> u64 {
        let seq = self.seq_counter;
        self.seq_counter += 1;
        seq
    }

    pub fn enqueue(&mut self, time: Tick, event: Event) {
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

        println!("All events in queue executed.");
    }

    pub fn get_time(self) -> (Tick, u64) {
        (self.clock, self.seq_counter)
    }

    pub fn node_ids(&self) -> &[NodeId] {
        &self.network.registry.clients
    }

    pub fn server_ids(&self) -> &[NodeId] {
        &self.network.registry.servers
    }
}