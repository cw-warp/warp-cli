use clap::builder::Str;
use serde::{Deserialize, Serialize};

use crate::utils::project_config::Network;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pipeline {
    pub name: String,
    pub config: PipelineConfig, 
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub cli_executable: String,
    pub cli_contract_subcommand: String,
    pub cli_args: String,
    pub cli_args_tx: String,
    pub cli_args_network: String,
    pub cli_args_store: String,
}