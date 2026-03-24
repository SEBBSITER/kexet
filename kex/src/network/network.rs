pub struct Network {
    topology: Topology,
    link_models: HashMap<(NodeId, NodeId), LinkModel>,
    partitions: PartitionState,
    rng: StdRng,
}

pub struct LinkModel {
    pub latency: LatencyModel,
    pub bandwidth_bps: u64,
    pub drop_rate: u64,
}

pub enum LatencyModel {
    Fixed(SimDuration),
}

impl Network {
    pub fn new(config: NetworkConfig) -> Network {
        
    }
}