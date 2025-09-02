use std::path::PathBuf;

use clap::Args;

use crate::{chains::chain_profile::ChainProfile, error::WarpError, executable::Executable, toolchains::pipeline::Pipeline, utils::project_config::ProjectConfig};


#[derive(Args)]
pub struct CreatePipelineCommand {
    pub name: String,
}

impl Executable for CreatePipelineCommand {
    fn execute(
        &self,
        project_root: Option<PathBuf>,
        config: Option<ProjectConfig>,
        _profile: &Box<dyn ChainProfile>,
    ) -> Result<(), WarpError> {
        let pipeline = Pipeline {
            name: self.name.clone(),
            config: crate::toolchains::pipeline::PipelineConfig { cli_executable: String::new(), cli_contract_subcommand: String::new(), cli_args: String::new(), cli_args_tx: String::new(), cli_args_network: String::new(), cli_args_store: String::new() }
        };
        let path = std::env::current_dir()?.join(format!("{}.toml", &self.name));
        std::fs::write(path, toml::to_string_pretty(&pipeline)?)?;
        println!("Created pipeline configuration: {}", &self.name);
        Ok(())
    }
}
