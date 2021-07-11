use crate::master_service::MasterService;

pub struct NodeJoinController {
    master_service: MasterService
}

impl NodeJoinController {
    pub fn new(master_service: MasterService) -> Self{
        Self{ master_service }
    }
}