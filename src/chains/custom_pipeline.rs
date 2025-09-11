use std::{
    fs::File, io::Write, net, path::PathBuf, process::{Command, Stdio}, time::Duration
};

use serde_json::Value;

use crate::{
    commands::config::NetworkConfig, cosmos::{keys_show::KeysShowResponse, tx_query::TxQueryResponse}, error::WarpError, toolchains::pipeline::Pipeline, utils::{file_util, project_config::Network}
};

use crate::utils::{command_util::CommandWithInput, project_config::ProjectConfig};

use super::chain_profile::ChainProfile;

pub struct CustomPipelineProfile {
    pub pipeline: Pipeline,
}

impl CustomPipelineProfile {
    pub fn new(pipeline: Pipeline) -> Self {
        Self { pipeline }
    }
}

impl ChainProfile for CustomPipelineProfile {
    fn get_executable_name(&self) -> String {
        self.pipeline.config.cli_executable.clone()
    }

    fn get_profile_name(&self) -> String {
        self.pipeline.name.clone()
    }

    fn get_common_cli_args<'a, 'b>(
        &self,
        tx: bool,
        network: bool,
        store: bool,
        config: &'b ProjectConfig,
    ) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        args.extend_from_slice(self.pipeline.config.cli_args.split_whitespace().collect::<Vec<&str>>().as_slice().iter().map(|s| s.to_string()).collect::<Vec<String>>().as_slice());
        if tx {
            args.extend_from_slice(self.pipeline.config.cli_args_tx.split_whitespace().collect::<Vec<&str>>().as_slice().iter().map(|s| s.to_string()).collect::<Vec<String>>().as_slice());
        }
        if network {
            args.extend_from_slice(self.pipeline.config.cli_args_network.split_whitespace().collect::<Vec<&str>>().as_slice().iter().map(|s| s.to_string()).collect::<Vec<String>>().as_slice());
            
        }
        args
    }

    fn get_key_info(
        &self,
        account_id: &str,
        password: Option<&str>,
        config: &ProjectConfig,
    ) -> Result<KeysShowResponse, WarpError> {
        let mut tx = Command::new(self.get_executable_name());
        tx.args(vec!["keys", "show", account_id])
            .args(self.get_common_cli_args(false, false, false, config))
            .stdout(Stdio::piped())
            .stdin(if password.is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            });
        let json_data: Vec<u8>;
        if let Some(pass) = password {
            let out = tx.call_process_with_input(pass)?;
            json_data = out.stdout;
        } else {
            let out = tx.output()?;
            let bytes = out.stdout.clone();
            json_data = bytes;
        }
        let response: KeysShowResponse = serde_json::from_slice(json_data.as_slice())?;
        Ok(response)
    }

    fn store_contract(
        &self,
        contract: &str,
        from: &str,
        password: Option<&str>,
        config: &ProjectConfig,
    ) -> Result<TxQueryResponse, WarpError> {
        let mut tx = Command::new(self.get_executable_name());
        tx.args(vec!["tx", "wasm", "store", contract, "--from", from])
            .args(self.get_common_cli_args(true, true, true, config))
            .stdout(Stdio::piped())
            .current_dir(ProjectConfig::find_project_root()?)
            .stdin(if password.is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            });
        let json_data: Vec<u8>;
        if let Some(pass) = password {
            let out = tx.call_process_with_input(pass)?;
            json_data = out.stdout;
        } else {
            let out = tx.output()?;
            let bytes = out.stdout.clone();
            json_data = bytes;
        }
        let response: TxQueryResponse = serde_json::from_slice(json_data.as_slice())?;
        if response.code != 0 {
            return Err(WarpError::TxFailed(response.txhash, response.raw_log));
        }
        let response = self.query_tx(&response.txhash, config)?;
        Ok(response)
    }

    fn instantiate_contract(
        &self,
        code_id: &str,
        from: &str,
        admin: &str,
        label: &str,
        init_msg: &str,
        coins: Option<String>,
        password: Option<&str>,
        config: &ProjectConfig,
    ) -> Result<TxQueryResponse, WarpError> {
        let mut tx = Command::new(self.get_executable_name());
        tx.args(vec![
            "tx",
            "wasm",
            "instantiate",
            code_id,
            init_msg,
            "--from",
            from,
            "--label",
            label,
            "--amount",
            &coins.unwrap_or_default(),
            "--admin",
            admin,
        ])
        .args(self.get_common_cli_args(true, true, false, config))
        .stdout(Stdio::piped())
        .stdin(if password.is_some() {
            Stdio::piped()
        } else {
            Stdio::inherit()
        });
        let json_data: Vec<u8>;
        if let Some(pass) = password {
            let out = tx.call_process_with_input(pass)?;
            json_data = out.stdout;
        } else {
            let out = tx.output()?;
            let bytes = out.stdout.clone();
            json_data = bytes;
        }
        let response: TxQueryResponse = serde_json::from_slice(json_data.as_slice())?;
        if response.code != 0 {
            return Err(WarpError::TxFailed(response.txhash, response.raw_log));
        }
        let response = self.query_tx(&response.txhash, config)?;
        Ok(response)
    }

    fn execute_contract(
        &self,
        contract_address: &str,
        msg: &str,
        from: &str,
        password: Option<&str>,
        config: &ProjectConfig,
    ) -> Result<TxQueryResponse, WarpError> {
        let mut tx = Command::new(self.get_executable_name());
        tx.args(vec![
            "tx",
            "wasm",
            "execute",
            contract_address,
            msg,
            "--from",
            from,
        ])
        .args(self.get_common_cli_args(true, true, false, config))
        .stdout(Stdio::piped())
        .stdin(if password.is_some() {
            Stdio::piped()
        } else {
            Stdio::inherit()
        });
        let json_data: Vec<u8>;
        if let Some(pass) = password {
            let out = tx.call_process_with_input(pass)?;
            json_data = out.stdout;
        } else {
            let out = tx.output()?;
            let bytes = out.stdout.clone();
            json_data = bytes;
        }
        let response: TxQueryResponse = serde_json::from_slice(json_data.as_slice())?;
        if response.code != 0 {
            return Err(WarpError::TxFailed(response.txhash, response.raw_log));
        }
        let response = self.query_tx(&response.txhash, config)?;
        Ok(response)
    }

    fn migrate_contract(
        &self,
        contract_address: &str,
        code_id: &str,
        from: &str,
        migrate_msg: &str,
        password: Option<&str>,
        config: &ProjectConfig,
    ) -> Result<TxQueryResponse, WarpError> {
        let mut tx = Command::new(self.get_executable_name());
        tx.args(vec![
            "tx",
            "wasm",
            "migrate",
            contract_address,
            code_id,
            migrate_msg,
            "--from",
            from,
        ])
        .args(self.get_common_cli_args(true, true, false, config))
        .stdout(Stdio::piped())
        .stdin(if password.is_some() {
            Stdio::piped()
        } else {
            Stdio::inherit()
        });
        let json_data: Vec<u8>;
        if let Some(pass) = password {
            let out = tx.call_process_with_input(pass)?;
            json_data = out.stdout;
        } else {
            let out = tx.output()?;
            let bytes = out.stdout.clone();
            json_data = bytes;
        }
        let response: TxQueryResponse = serde_json::from_slice(json_data.as_slice())?;
        if response.code != 0 {
            return Err(WarpError::TxFailed(response.txhash, response.raw_log));
        }
        let response = self.query_tx(&response.txhash, config)?;
        Ok(response)
    }

    // TODO: Make this ugly thing go away once a better solution is confirmed to be working
    fn query_tx(
        &self,
        tx_hash: &str,
        config: &ProjectConfig,
    ) -> Result<TxQueryResponse, WarpError> {
        let mut retries = 10;
        loop {
            let cmd = Command::new(self.get_executable_name())
                .args(vec!["q", "tx", tx_hash])
                .args(self.get_common_cli_args(false, true, false, config))
                .stdin(Stdio::inherit())
                .output()?;
            let tx = cmd.stdout;
            let stderr = cmd.stderr;
            if !stderr.is_empty() && retries > 0 {
                // crude but will do for beta
                retries -= 1;
                std::thread::sleep(Duration::from_millis(1958));
                continue;
            }
            let response: TxQueryResponse = serde_json::from_slice(tx.as_slice())?;
            if response.code != 0 {
                return Err(WarpError::TxFailed(response.txhash, response.raw_log));
            }
            return Ok(response);
        }
    }

    fn query_contract_smart(
        &self,
        contract: &str,
        query: &str,
        config: &ProjectConfig,
    ) -> Result<Value, WarpError> {
        let cmd = Command::new(self.get_executable_name())
            .args(vec![
                "q",
                "wasm",
                "contract-state",
                "smart",
                contract,
                query,
            ])
            .args(self.get_common_cli_args(false, true, false, config))
            .stdin(Stdio::inherit())
            .output()?;
        let tx = cmd.stdout;
        if cmd.stderr.len() > 0 {
            let msg = String::from_utf8(cmd.stderr)?;
            return Err(WarpError::UnderlyingCliError(msg));
        }
        let response: Value = serde_json::from_slice(tx.as_slice())?;
        Ok(response)
    }

    fn init_project(&self, dir: &PathBuf) -> Result<(), WarpError> {
        println!("Initializing new workspace...");
        let cmd = Command::new("git")
            .arg("clone")
            .arg("https://github.com/cw-warp/workspace-template.git")
            .arg(dir.as_os_str())
            .arg("--branch")
            .arg("chain/juno")
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
        if cmd.success() {
            return Ok(());
        } else {
            return Err(WarpError::InitFailed);
        }
    }

    fn new_contract(
        &self,
        contract_name: &str,
        contract_dir: &PathBuf,
        project_root: &PathBuf,
    ) -> Result<(), WarpError> {
        println!("[1/2] Downloading contract files...");
        std::fs::create_dir_all(contract_dir.clone())?;
        let clone = std::process::Command::new("git")
            .args(vec![
                "clone",
                "--depth=1",
                "https://github.com/cw-warp/contract-template.git",
                contract_dir.clone().as_os_str().to_str().unwrap(),
                "--branch",
                "chain/juno",
                "-q",
            ])
            .spawn()?
            .wait()?;
        if !clone.success() {
            return Err(WarpError::ContractTemplateCloneFailed);
        }

        std::fs::remove_dir_all(contract_dir.clone().join(".git"))?;
        std::fs::remove_file(contract_dir.clone().join("README.md"))?;
        let cargo_path = contract_dir.clone().join("Cargo.toml");
        file_util::replace_in_file(cargo_path, "<CONTRACT_NAME>", &contract_name)?;

        let lib_path = contract_dir.clone().join("src").join("contract.rs");
        file_util::replace_in_file(lib_path, "<CONTRACT_NAME>", &contract_name)?;

        let schema_path = contract_dir
            .clone()
            .join("src")
            .join("bin")
            .join("schema.rs");
        file_util::replace_in_file(schema_path, "<CONTRACT_NAME>", &contract_name)?;

        let shared_path = project_root.clone().join("packages").join("shared");
        let msg_path = shared_path
            .clone()
            .join("src")
            .join(&contract_name)
            .join("msg.rs");
        std::fs::create_dir_all(msg_path.clone().parent().unwrap())?;
        let mod_path = msg_path.clone().parent().unwrap().join("mod.rs");
        std::fs::write(msg_path, crate::consts::MSG_FILE)?;
        std::fs::write(mod_path, "pub mod msg;")?;
        let lib_path = shared_path.clone().join("src").join("lib.rs");
        let mut lib_file = File::options().write(true).append(true).open(&lib_path)?;
        println!("Shared Lib: {}", &lib_path.to_str().unwrap());
        writeln!(&mut lib_file, "pub mod {};", &contract_name)?;
        println!("[2/2] Building the workspace...");
        std::process::Command::new("cargo")
            .arg("build")
            .current_dir(project_root)
            .spawn()?
            .wait()?;
        Ok(())
    }

    fn get_node_docker_command(&self, container: Option<String>, config: &ProjectConfig) -> String {
        format!("docker run{} -it --name {} -p 1317:1317 -p 26656:26656 -p 26657:26657 -e STAKE_TOKEN=ujunox -e UNSAFE_CORS=true ghcr.io/cosmoscontracts/juno:14.1.0 ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y",
            //std::env::current_dir().unwrap().to_str().unwrap(), 
            if config.tests.persist_image { "" } else { " --rm" },
            container.clone().unwrap_or_else(|| config.tests.test_container_name.clone())
        )
    }

    fn network_params(&self, network_config: &NetworkConfig) -> Network {
        match network_config {
            NetworkConfig::Mainnet => Network {
                profile: self.get_profile_name(),
                chain_id: "juno-1".to_owned(),
                rpc_url: "https://juno-rpc.polkachu.com:443".to_owned(),
                denom: "ujuno".to_owned(),
                gas_prices: Some("0.002ujuno".to_owned()),
            },
            NetworkConfig::Testnet => Network {
                profile: self.get_profile_name(),
                chain_id: "uni-6".to_owned(),
                rpc_url: "https://testnet.sentry.tm.injective.network:443".to_owned(),
                denom: "ujunox".to_owned(),
                gas_prices: Some("0.0002ujunox".to_owned()),
            },
            NetworkConfig::Local => Network {
                profile: self.get_profile_name(),
                chain_id: "testing".to_owned(),
                rpc_url: "http://localhost:26657".to_owned(), // TODO: Add local node URL
                denom: "ujunox".to_owned(),
                gas_prices: Some("0.000001ujunox".to_owned()),
            },
        }
    }

    fn get_initialized_address(&self, tx: &TxQueryResponse) -> String {
        
        tx.logs
            .first()
            .unwrap()
            .events
            .get(0)
            .unwrap()
            .attributes
            .get(0)
            .unwrap()
            .value
            .clone()
    }

    fn init_frontend(&self, _dir: &PathBuf) -> Result<(), WarpError> {
        todo!();
        // let mut cmd = Command::new("git")
        //     .arg("clone")
        //     .arg("https://github.com/xion-warp/frontend")
        //     .current_dir(dir)
        //     .stdout(Stdio::null())
        //     .spawn()?;
        // let cmd = cmd.wait()?;
        // if !cmd.success() {
        //     return Err(WarpError::InitFailed);
        // }
        // println!("{} - run: {}", "Frontend initialized.", "yarn && yarn dev".bright_yellow());
        // Ok(())
    }
}
