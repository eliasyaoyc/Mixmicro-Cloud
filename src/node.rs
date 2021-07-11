use crate::{Result, Lifecycle};
use crate::request::{ElectionResp, AliveResp, VictoryResp};
use crate::discovery::Discovery;
use crate::fault_detection::FaultDetection;
use crate::elect_master_service::ElectMasterService;
use crate::no_master_block_service::NoMasterBlockService;
use crate::master_service::MasterService;
use crate::cluster::Cluster;

pub struct Node {
    id: &'static str,
    discovery: Discovery,
    cluster: Cluster,
}

impl Lifecycle for Node {
    fn start(&self) {
        self.discovery.start();
    }

    fn stop(&self) {
        todo!()
    }
}

impl Node {
    pub fn new(node_id: &str) -> Self {
        Node {
            id: node_id,
            discovery: Discovery {
                node_fault_detection: FaultDetection {},
                master_fault_detection: FaultDetection {},
                elect_master_service: ElectMasterService {},
                no_master_block_service: NoMasterBlockService {},
                master_service: MasterService {},
            },
            cluster: Cluster::new(),
        }
    }
    // pub fn election(&self) -> Result<ElectionResp> {}

    // pub fn alive(&self) -> Result<AliveResp> {}

    // pub fn victory(&self) -> Result<VictoryResp> {}

    pub fn is_master(&self) -> bool {
        true
    }

    pub fn rejoin(&self, reason: &str) {}
}