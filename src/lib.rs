use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import the RPC client
// use clap::Parser;
use integritee_cli::{commands, Cli, trusted_cli::TrustedCli};
use integritee_cli::commands::Commands;
use integritee_cli::trusted_cli::{PayAsBidCommand, TrustedBaseCommand, TrustedCommand};

#[pymodule]
fn integritee_cli_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}

#[pyfunction]
fn run_cli(node_url: String, node_port: String, worker_url: String, trusted_worker_port: String, _command: String) -> PyResult<()> {
    println!("Run Cli");
    let cli = pay_as_pid_cmd(node_url, node_port, worker_url, trusted_worker_port);

    commands::match_command(&cli).unwrap();
    Ok(())
}

fn pay_as_pid_cmd(node_url: String, node_port: String, worker_url: String, trusted_worker_port: String) -> Cli {
    Cli {
        node_url,
        node_port,
        worker_url,
        trusted_worker_port,
        command: Commands::Trusted(TrustedCli{
            // random mrenclave, has to be replaced with one that has been fetched from the chain.
            mrenclave: "4GMb72Acyg8hnnnGEJ89jZK5zxNC4LvSe2ME96wLRV6J".to_string(),
            shard: None,
            // signers and accounts starting with `//` will be recognized as dev-seeds and can
            // always be used without first creating them in the keystore.
            xt_signer: "//Alice".to_string(),
            direct: false,
            command: TrustedCommand::BaseTrusted(TrustedBaseCommand::PayAsBid(PayAsBidCommand {
                account: "//Alice".to_string(),
                orders_string: "".to_string(),
            })),
        }),
    }
}

