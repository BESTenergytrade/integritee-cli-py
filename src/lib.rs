use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import the RPC client
// use clap::Parser;
use integritee_cli::{commands, Cli, trusted_cli::TrustedCli};
use integritee_cli::commands::{BaseCommand, Commands};
use integritee_cli::trusted_cli::{PayAsBidCommand, TrustedBaseCommand, TrustedCommand};

#[pymodule]
fn integritee_cli_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}

#[pyfunction]
fn run_cli(node_url: String, node_port: String, worker_url: String, trusted_worker_port: String, _command: String) -> PyResult<()> {
    println!("Run Cli");

    let cli = Cli {
        node_url,
        node_port,
        worker_url,
        trusted_worker_port,
        command: new_account_cmd(),
        // command: pay_as_bid_cmd(),
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
    Commands::Trusted(TrustedCli{
        // random mrenclave, has to be replaced with one that has been fetched from the chain.
        mrenclave: "4GMb72Acyg8hnnnGEJ89jZK5zxNC4LvSe2ME96wLRV6J".to_string(),
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
    Commands::Trusted(TrustedCli{
        // random mrenclave, has to be replaced with one that has been fetched from the chain.
        mrenclave: "4GMb72Acyg8hnnnGEJ89jZK5zxNC4LvSe2ME96wLRV6J".to_string(),
        shard: None,
        // signers and accounts starting with `//` will be recognized as dev-seeds and can
        // always be used without first creating them in the keystore.
        xt_signer: "//Alice".to_string(),
        direct: true,
        command: TrustedCommand::BaseTrusted(TrustedBaseCommand::PayAsBid(PayAsBidCommand {
            account: "//Alice".to_string(),
            orders_string: "".to_string(),
        })),
    })
}

