use std::path::PathBuf;

pub use clap::Args;
use owo_colors::OwoColorize;

use crate::{
    chains::chain_profile::ChainProfile, error::WarpError, executable::Executable,
    utils::project_config::ProjectConfig,
};

#[derive(Args)]
pub struct SchemaCommand {}

impl Executable for SchemaCommand {
    async fn execute(
        &self,
        project_root: Option<PathBuf>,
        _config: Option<ProjectConfig>,
        _profile: &Box<dyn ChainProfile>,
    ) -> Result<(), WarpError> {
        if project_root.is_none() {
            return Err(WarpError::ProjectFileNotFound);
        };
        let project_root = project_root.unwrap();
        //let config = config.unwrap();

        let global_schema_dir = project_root.join("schema");
        if !global_schema_dir.exists() {
            std::fs::create_dir(&global_schema_dir)?;
        }

        let contracts_dir = project_root.join("contracts");
        let contracts_paths = std::fs::read_dir(&contracts_dir)?.filter_map(|x| match x {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            }
            Err(_) => None,
        });

        for path in contracts_paths {
            let contract_name = path.file_name().unwrap().to_string_lossy().to_string();
            let command = std::process::Command::new("cargo")
                .current_dir(&path)
                .args(vec!["run", "--bin", "schema"])
                .output();
            if command.is_err() {
                println!("{}{}{}",
                    "❌  Error generating schema for contract '".bright_yellow(),
                    contract_name.bright_red(),
                    "'. Please ensure the contract is valid and has a schema generation script. Skipping...".bright_yellow(),
                );
                continue;
            }
            let command = command.unwrap();

            if !command.status.success() {
                println!(
                    "{}{}{}",
                    "❌  Error generating schema for contract '".bright_yellow(),
                    contract_name.bright_red(),
                    "': ".bright_yellow(),
                );
                continue;
            }
            let schema_path = path.join("schema").join(format!("{}.json", &contract_name));

            let schema_destination = global_schema_dir.join(format!("{}.json", &contract_name));
            if schema_destination.exists() {
                std::fs::remove_file(&schema_destination)?;
            }
            std::fs::copy(&schema_path, &schema_destination)?;

            println!(
                "{}{}{}",
                "✔️  Schema for contract '".bright_yellow(),
                contract_name.bright_green(),
                "' generated successfully!".bright_yellow(),
            )
        }
        Ok(())
    }
}
