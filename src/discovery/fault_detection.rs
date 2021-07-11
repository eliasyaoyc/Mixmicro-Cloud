use std::fmt::{Debug, Formatter};
use crate::node::Node;
use crate::{Lifecycle, Error};
use crate::request::Response;
use std::rc::Rc;

//! Fault detection in the cluster, it has two type detector as following:
//! 1. NodesFaultDetection
//!    Ping requests are sent to all of the nodes in the cluster except themselves.
//!    Note that this only applies to the master node. If the number of successful
//!    responses to requests is less than the value of the discovery.zen.minimum_master_nodes,
//!    then the node drops the master status and calls rejoin to re-elect master.
//!
//! 2. MasterFaultDetection
//!    This only applies to the no master node. it will pings the master node periodically to see if its alive.
//!    Send rejoin request to another nodes if master node is dead.
pub trait FaultDetection: Lifecycle {
    fn detect(&self, node: &Node);
    fn handle_response(&self, resp: Response);
    fn handle_error(&self, err: Error);
}

pub fn generate_detection(node: &Node) -> Box<dyn FaultDetection> {
    if node.is_master() {
        return Box::new(NodesFaultDetection { retry: 0, node: Rc::new(Node::new()) });
    }
    Box::new(MasterFaultDetection { retry: 0, node: Rc::new(Node::new()) })
}

pub struct NodesFaultDetection {
    pub retry: u32,
    pub node: Rc<Node>,
}

impl Lifecycle for NodesFaultDetection {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}

impl FaultDetection for NodesFaultDetection {
    fn detect(&self, node: &Node) {
        todo!()
    }

    fn handle_response(&self, resp: Response) {
        todo!()
    }

    fn handle_error(&self, err: Error) {
        todo!()
    }
}

pub struct MasterFaultDetection {
    pub retry: u32,
    pub node: Rc<Node>,
}

impl Lifecycle for MasterFaultDetection {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}

impl FaultDetection for MasterFaultDetection {
    fn detect(&self, node: &Node) {
        todo!()
    }

    fn handle_response(&self, resp: Response) {
        todo!()
    }

    fn handle_error(&self, err: Error) {
        self.node.rejoin("master left...");
    }
}