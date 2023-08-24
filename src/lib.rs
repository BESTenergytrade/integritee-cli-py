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

fn find_command(command_name: &str, params: &[String]) -> Commands {
    match command_name {
        "new_account_cmd" => new_account_cmd(),
        "new_trusted_account_cmd" => new_trusted_account_cmd(&params),
        "pay_as_bid_cmd" => pay_as_bid_cmd(&params),
        "get_market_results_cmd" => get_market_results_cmd(&params),
        "pay_as_bid_proof_cmd" => pay_as_bid_proof_cmd(&params),
        "verify_proof_cmd" => verify_proof_cmd(&params),
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
    params: Vec<String>,
) -> PyResult<()> {
    let cli = Cli {
        node_url,
        node_port,
        worker_url,
        trusted_worker_port,
        command: find_command(&command_name, &params),
    };

    match command_name.as_str() {
        "new_account_cmd" => new_account_cmd(),
        "new_trusted_account_cmd" => new_trusted_account_cmd(&params),
        "pay_as_bid_cmd" => pay_as_bid_cmd(&params),
        "get_market_results_cmd" => get_market_results_cmd(&params),
        "pay_as_bid_proof_cmd" => pay_as_bid_proof_cmd(&params),
        "verify_proof_cmd" => verify_proof_cmd(&params),
        _ => panic!("Invalid command name"),
    };

    commands::match_command(&cli).unwrap();
    Ok(())
}

/// Create a new account
fn new_account_cmd() -> Commands {
    Commands::Base(BaseCommand::NewAccount)
}

/// Create a new trusted account in the enclave.
fn new_trusted_account_cmd(params: &[String]) -> Commands {
    let mrenclave = &params[0];
    Commands::Trusted(TrustedCli {
        // use mrenclave that has been fetched from the chain.
        mrenclave: mrenclave.to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        // `//Alice` is not actually the account to be created here, 
        // but it is used as signer for the trusted call
        xt_signer: "//Alice".to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::NewAccount),
    })
}

fn pay_as_bid_cmd(params: &[String]) -> Commands {
    let mrenclave = &params[0];
    let account = &params[1];
    let orders_string = &params[2];
    Commands::Trusted(TrustedCli {
        // use mrenclave that has been fetched from the chain.
        mrenclave: mrenclave.to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: account.to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::PayAsBid(PayAsBidCommand {
            account: account.to_string(),
            orders_string: orders_string.to_string(),
        })),
    })
}

fn get_market_results_cmd(params: &[String]) -> Commands {
    let mrenclave = &params[0];
    let account = &params[1];
    let timestamp = &params[2];
    Commands::Trusted(TrustedCli {
        // use mrenclave that has been fetched from the chain.
        mrenclave: mrenclave.to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: account.to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::GetMarketResults(
            GetMarketResultsCommand {
                account: account.to_string(),
                timestamp: timestamp.to_string(),
            },
        )),
    })
}

fn pay_as_bid_proof_cmd(params: &[String]) -> Commands {
    let mrenclave = &params[0];
    let account = &params[1];
    let timestamp = &params[2];
    let actor_id = &params[3];
    Commands::Trusted(TrustedCli {
        // use mrenclave that has been fetched from the chain.
        mrenclave: mrenclave.to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: account.to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::PayAsBidProof(
            PayAsBidProofCommand {
                account: account.to_string(),
                timestamp: timestamp.to_string(),
                actor_id: actor_id.to_string(),
            },
        )),
    })
}

fn verify_proof_cmd(params: &[String]) -> Commands {
    let mrenclave = &params[0];
    let account = &params[1];
    let merkle_proof_json = &params[2];
    Commands::Trusted(TrustedCli {
        mrenclave: mrenclave.to_string(),
        shard: None,
        xt_signer: account.to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::VerifyProof(
            VerifyMerkleProofCommand {
                merkle_proof_json: merkle_proof_json.to_string(),
            },
        )),
    })
}
