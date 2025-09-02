pub mod create;

use std::path::PathBuf;

use clap::{Args, Command, Subcommand};

use crate::{chains::chain_profile::ChainProfile, commands::pipeline::create::CreatePipelineCommand, error::WarpError, executable::Executable, utils::project_config::ProjectConfig};


#[derive(Subcommand)]
pub enum PipelineCommand {
    /// Create a new pipeline configuration file template
    Create(CreatePipelineCommand), 
}

impl Executable for PipelineCommand {
    fn execute(
        &self,
        project_root: Option<PathBuf>,
        config: Option<ProjectConfig>,
        profile: &Box<dyn ChainProfile>,
    ) -> Result<(), WarpError> {
        match self {
            PipelineCommand::Create(cmd) => cmd.execute(project_root, config, profile),
        }
    }
}