use crate::{commands::schema::SchemaCommand, executable::Executable};
use cw_swaggy::executable::{Executable as SwaggyExecutable, ExecutionContext};


#[derive(clap::Args, Debug, Clone, PartialEq, Eq)]
pub struct DocgenCommand {}

impl Executable for DocgenCommand {
    async fn execute(
        &self,
        project_root: Option<std::path::PathBuf>,
        config: Option<crate::utils::project_config::ProjectConfig>,
        profile: &Box<dyn crate::chains::chain_profile::ChainProfile>,
    ) -> Result<(), crate::error::WarpError> {
        // Placeholder for documentation generation logic
        println!("Generating documentation...");
        let root_path = project_root.unwrap();
        // 1. Schema generation
        SchemaCommand { 

        }.execute(Some(root_path.clone()), config, profile).await?;


        // 2. Docgen execution - build
        let ctx = ExecutionContext::try_load();
        let ctx = ctx.map_err(|e| crate::error::WarpError::DocgenError(e.to_string()))?;
        cw_swaggy::commands::build::BuildCmd {
            schema: root_path.join("schema"),
        }
            .execute(&ctx)
            .await
            .map_err(|e| crate::error::WarpError::DocgenError(e.to_string()))?;
        
        // 3. Docgen execution - serve
        cw_swaggy::commands::serve::ServeCmd {
            schema: root_path.join("openapi.json"),
            port: 8008,
            wasm: None,
        }
        .execute(&ctx)
        .await
        .map_err(|e| crate::error::WarpError::DocgenError(e.to_string()))?;
        Ok(())
    }
}