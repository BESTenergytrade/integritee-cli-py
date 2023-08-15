use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import the RPC client
// use clap::Parser;
use integritee_cli::commands::{BaseCommand, Commands};
use integritee_cli::trusted_cli::{
    GetMarketResultsCommand, PayAsBidCommand, PayAsBidProofCommand, TrustedBaseCommand,
    TrustedCommand, VerifyMerkleProofCommand,
};
use integritee_cli::{commands, trusted_cli::TrustedCli, Cli};

#[pymodule]
fn integritee_cli_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}

fn find_command(command_name: &str) -> Commands {
    match command_name {
        "new_account_cmd" => new_account_cmd(),
        "pay_as_bid_cmd" => pay_as_bid_cmd(),
        "get_market_results_cmd" => get_market_results_cmd(),
        "pay_as_bid_proof_cmd" => pay_as_bid_proof_cmd(),
        "verify_proof_cmd" => verify_proof_cmd(),
        _ => panic!("Invalid command name"),
    }
}

#[pyfunction]
fn run_cli(
    node_url: String,
    node_port: String,
    worker_url: String,
    trusted_worker_port: String,
    command_name: String,
) -> PyResult<()> {
    let cli = Cli {
        node_url,
        node_port,
        worker_url,
        trusted_worker_port,
        command: find_command(&command_name),
    };

    commands::match_command(&cli).unwrap();
    Ok(())
}

/// Create a new account
fn new_account_cmd() -> Commands {
    Commands::Base(BaseCommand::NewAccount)
}

/// Create a new trusted account in the enclave.
fn new_trusted_account_cmd() -> Commands {
    Commands::Trusted(TrustedCli {
        // random mrenclave, has to be replaced with one that has been fetched from the chain.
        mrenclave: "Az1EL1mXZokRKKaBkmhcKiQXYZk3Q24C9nw8TiNnpwTL".to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: "//Alice".to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::NewAccount),
    })
}

// just a skeleton to see if the cli understands it. It will never return a successful result.
// We need to fill in some actual meaningful values.
fn pay_as_bid_cmd() -> Commands {
    Commands::Trusted(TrustedCli {
        // random mrenclave, has to be replaced with one that has been fetched from the chain.
        mrenclave: "Az1EL1mXZokRKKaBkmhcKiQXYZk3Q24C9nw8TiNnpwTL".to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: "//Alice".to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::PayAsBid(PayAsBidCommand {
            account: "//Alice".to_string(),
            orders_string:"[{\"id\":0,\"order_type\":\"ask\",\"time_slot\":\"2022-03-04T05:06:07+00:00\",\"actor_id\":\"actor_0\",\"cluster_index\":0,\"energy_kwh\":5,\"price_euro_per_kwh\":0.19},{\"id\":1,\"order_type\":\"bid\",\"time_slot\":\"2022-03-04T05:06:07+00:00\",\"actor_id\":\"actor_1\",\"cluster_index\":0,\"energy_kwh\":8.8,\"price_euro_per_kwh\":0.23}]".to_string(),
        })),
    })
}

fn get_market_results_cmd() -> Commands {
    Commands::Trusted(TrustedCli {
        // random mrenclave, has to be replaced with one that has been fetched from the chain.
        mrenclave: "Az1EL1mXZokRKKaBkmhcKiQXYZk3Q24C9nw8TiNnpwTL".to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: "//Alice".to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::GetMarketResults(
            GetMarketResultsCommand {
                account: "//Alice".to_string(),
                timestamp: "2022-03-04T05:06:07+00:00".to_string(),
            },
        )),
    })
}

fn pay_as_bid_proof_cmd() -> Commands {
    Commands::Trusted(TrustedCli {
        // random mrenclave, has to be replaced with one that has been fetched from the chain.
        mrenclave: "9PPeGELLdD9Uw1mVJbUGTeRpGzPBGb1bdEk6TCL4pPCE".to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: "//Alice".to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::PayAsBidProof(
            PayAsBidProofCommand {
                account: "//Alice".to_string(),
                timestamp: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_0".to_string(),
            },
        )),
    })
}

fn verify_proof_cmd() -> Commands {
    Commands::Trusted(TrustedCli {
        // random mrenclave, has to be replaced with one that has been fetched from the chain.
        mrenclave: "9PPeGELLdD9Uw1mVJbUGTeRpGzPBGb1bdEk6TCL4pPCE".to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: "//Alice".to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::VerifyProof(VerifyMerkleProofCommand {
            merkle_proof_json: "{\"root\":\"0x674b938e15b718ab5831dd027948e6252df15e0cde89072279ceb0633795e457\",\"proof\":[\"0x99aad8dbdf1379081361390b6bd491cbb8cc29d5406288d3665beab2b00b6d70\"],\"number_of_leaves\":2,\"leaf_index\":0,\"leaf\":[0,0,0,0,0,0,0,0,1,100,50,48,50,50,45,48,51,45,48,52,84,48,53,58,48,54,58,48,55,43,48,48,58,48,48,28,97,99,116,111,114,95,48,1,0,0,0,0,0,0,0,0,0,0,20,64,82,184,30,133,235,81,200,63]}".to_string(),
        })),
    })
}
