use std::collections::HashMap;
use crate::common::{Message, NodeId, Tick};
use crate::nemesis::{Nemesis, NemesisVerdict};
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

impl LinkConfig {
    pub fn perfect_link() -> LinkConfig {
        Self {
            latency: LatencyModel::Fixed(Tick(0)),
            loss_rate: 0.0,
            reorder_rate: 0.0,
            bandwidth_bps: None,
        }
    }
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

pub struct NodeRegistry {
    pub(crate) servers: Vec<NodeId>,
    pub(crate) clients: Vec<NodeId>,
}

impl NodeRegistry {
    fn new() -> NodeRegistry {
        Self {
            servers: Vec::new(),
            clients: Vec::new(),
        }
    }

    pub fn all_nodes(&self) -> impl Iterator<Item = &NodeId> {
        self.servers.iter().chain(self.clients.iter())
    }

    // TODO: Add error handling
    fn add_server(&mut self, node: NodeId) {
        // TODO: Check for uniqueness
        self.servers.push(node);
        //Ok(())
    }

    // TODO: Add error handling
    fn add_client(&mut self, node: NodeId) {
        // TODO: Check for uniqueness
        self.clients.push(node);
        //Ok(())
    }
}

pub enum TopologyType {
    Star,
    Mesh,
    Ring,
    Tree,
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
    pub(crate) registry: NodeRegistry,
    topology: Topology,
    nemesis: Nemesis,
    rng: StdRng,
}

impl Network {
    pub fn new(topology_type: TopologyType, number_clients: usize, number_servers: usize) -> Network {

        // TO easily manage ID
        let total_nodes = number_clients + number_servers;

        // TODO: Find better solution for ID registration. Currently allows for mismatches
        let mut registry= NodeRegistry::new();
        // First 0..<number_servers is id for servers
        for id in 0..number_servers {
            registry.add_server(id as u32);
        }
        // number_servers..<total_nodes is id for clients
        for id in number_servers..number_clients {
            registry.add_client(id as u32);
        }

        let links = match topology_type {
            TopologyType::Star => Self::create_star_topology(
                number_clients,
                number_servers,
            ),
            TopologyType::Mesh => Self::create_mesh_topology(
                number_clients,
                number_servers,
                total_nodes,
            ),
            TopologyType::Ring => Self::create_ring_topology(
                number_clients,
                number_servers,
                total_nodes,
            ),
            TopologyType::Tree => Self::create_balanced_tree_topology(
                number_clients,
                number_servers,
                total_nodes,
            ),
        };

        let topology = Topology { links, default_link: None };
        let nemesis = Nemesis::new();
        let rng = rand::make_rng();

        Self {
            registry,
            topology,
            nemesis,
            rng,
        }
    }

    fn create_star_topology(
        number_clients: usize,
        number_servers: usize,
    ) -> HashMap<(NodeId, NodeId), LinkConfig> {
        let mut links = HashMap::new();
        for s_id in 0..number_servers {
            for c_idx in number_servers..number_clients {
                let config = LinkConfig::perfect_link();
                links.insert((s_id as NodeId, c_idx as NodeId), config.clone());
                links.insert((c_idx as NodeId, s_id as NodeId), config.clone());
            }
        }
        links
    }

    fn create_mesh_topology(
        number_clients: usize,
        number_servers: usize,
        total_nodes: usize,
    ) -> HashMap<(NodeId, NodeId), LinkConfig> {
        // TODO: Implement mesh topology
        let mut links = HashMap::new();
        links
    }

    fn create_ring_topology(
        number_clients: usize,
        number_servers: usize,
        total_nodes: usize,
    ) -> HashMap<(NodeId, NodeId), LinkConfig> {
        let mut links = HashMap::new();
        links
    }

    fn create_balanced_tree_topology(
        number_clients: usize,
        number_servers: usize,
        total_nodes: usize,
    ) -> HashMap<(NodeId, NodeId), LinkConfig> {
        // TODO: Implement tree topology
        let mut links = HashMap::new();
        links
    }
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