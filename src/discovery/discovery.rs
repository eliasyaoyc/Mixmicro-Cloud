use std::fmt::{Debug, Formatter};
use crate::Lifecycle;
use crate::fault_detection::FaultDetection;
use crate::elect_master_service::ElectMasterService;
use crate::no_master_block_service::NoMasterBlockService;
use crate::master_service::MasterService;

pub struct Discovery {
    pub node_fault_detection: FaultDetection,
    pub master_fault_detection: FaultDetection,
    pub elect_master_service: ElectMasterService,
    pub no_master_block_service: NoMasterBlockService,
    pub master_service: MasterService,
}

impl Debug for Discovery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("discovery")
            .field("nodeFaultDetection", &self.node_fault_detection)
            .field("masterFaultDetection", &self.master_fault_detection)
            .field("electMasterService", &self.elect_master_service)
            .field("noMasterBlockService", &self.no_master_block_service)
            .field("masterService", &self.master_service)
            .finish()
    }
}

impl Lifecycle for Discovery {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}
