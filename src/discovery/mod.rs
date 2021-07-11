mod node;
mod error;
mod config;
mod discovery;
mod request;
mod fault_detection;
mod elect_master_service;
mod no_master_block_service;
mod master_service;
mod node_join_controller;
mod cluster;

pub use error::BullyError as Error;
pub use error::Result;

pub trait Lifecycle {
    fn start(&self) {}

    fn stop(&self) {}
}