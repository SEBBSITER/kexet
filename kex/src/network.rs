use std::collections::HashMap;
use crate::common::{Message, NodeId};
use crate::nemesis::{Nemesis, NemesisVerdict};
use crate::simulator::Tick;
use rand::rngs::{StdRng};
use rand::{Rng, RngExt};
use rand_distr::{Distribution, Normal};

pub struct PendingDelivery {
    pub time: Tick,
    pub from: NodeId,
    pub to: NodeId,
    pub message: Message,
}

#[derive(Clone)]
pub struct LinkConfig {
    pub latency: LatencyModel,
    pub loss_rate: f64,
    pub reorder_rate: f64,
    pub bandwidth_bps: Option<u64>,
}

#[derive(Clone)]
pub enum LatencyModel {
    Fixed(Tick),
    Uniform { min: Tick, max: Tick },
    Normal { mean_ticks: f64, std_ticks: f64 },
    Bimodal {fast: Tick, slow: Tick, slow_probability: f64 },
}

impl LatencyModel {
    pub fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Tick {
        match self {
            LatencyModel::Fixed(tick) => *tick,
            LatencyModel::Uniform { min, max } => {
                Tick(rng.random_range(min.0..=max.0))
            }
            LatencyModel::Normal { mean_ticks, std_ticks } => {
                let normal = Normal::new(*mean_ticks, *std_ticks).unwrap();
                Tick(normal.sample(rng) as u64)
            }
            LatencyModel::Bimodal { fast, slow, slow_probability } => {
                if rng.random::<f64>() < *slow_probability {
                    *slow
                } else {
                    *fast
                }
            }
        }
    }
}

pub struct Topology {
    links: HashMap<(NodeId, NodeId), LinkConfig>,
    default_link: Option<LinkConfig>,
}

impl Topology {
    // TODO: Implement reading topology from configuration
    pub fn new() {}

    pub fn get_link(&self, from: NodeId, to: NodeId) -> Option<&LinkConfig> {
        self.links.get(&(from, to)).or(self.default_link.as_ref())
    }
}

pub struct Network {
    topology: Topology,
    nemesis: Box<dyn Nemesis>,
    rng: StdRng,
}

impl Network {
    pub fn process_send(
        &mut self,
        from: NodeId,
        to: NodeId,
        msg: Message,
        now: Tick,
    ) -> Vec<PendingDelivery> {
        // Topology: does the link exist
        let link = match self.topology.get_link(from, to).cloned() {
            Some(l) => l,
            None => return vec![], // note silent drop
        };

        // Nemesis filter
        let msg = match self.nemesis.on_send(from, to, msg, now) {
            NemesisVerdict::Allow(msg) => msg,
            NemesisVerdict::Drop => return vec![], // silent drop
            NemesisVerdict::Delay(msg, extra) => {
                return self.make_delivery(from, to, msg, now, link, extra);
            }
        };

        // Link-level loss
        if self.rng.random::<f64>() < link.loss_rate {
            return vec![]; // silent drop
        }

        self.make_delivery(from, to, msg, now, link, Tick(0))
    }

    fn make_delivery(
        &mut self,
        from: NodeId,
        to: NodeId,
        msg: Message,
        now: Tick,
        link: LinkConfig,
        extra_delay: Tick,
    ) -> Vec<PendingDelivery> {
        let latency = link.latency.sample(&mut self.rng);
        let delivery_time = Tick(now.0 + latency.0 + extra_delay.0);

        vec![PendingDelivery { time: delivery_time, from, to, message: msg }]
    }
}