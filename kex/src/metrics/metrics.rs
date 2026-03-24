pub trait Metrics {
    fn loss(idx: NodeId);
    fn convergence(idx: NodeId);
}