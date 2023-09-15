use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use pyo3::PyResult;

// Import the RPC client
// use clap::Parser;
use integritee_cli::commands::{BaseCommand, Commands};
use integritee_cli::trusted_cli::{
    GetMarketResultsCommand, PayAsBidCommand, PayAsBidProofCommand, TrustedBaseCommand,
    TrustedCommand, VerifyMerkleProofCommand,
};
use integritee_cli::CliResultOk;
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
    py: Python,
) -> PyResult<Py<PyDict>> {
    let cli = Cli {
        node_url,
        node_port,
        worker_url,
        trusted_worker_port,
        command: find_command(&command_name, &params),
    };

    let cmd_output = commands::match_command(&cli);

    match cmd_output {
        Ok(output) => match output {
            CliResultOk::Matches(matches) => {
                let matches_dict = PyDict::new(py);

                let matches_list = PyList::new(
                    py,
                    matches.matches.into_iter().map(|mtch| {
                        let dict = PyDict::new(py);
                        let _ = dict.set_item("time", mtch.energy_kwh);
                        let _ = dict.set_item("bid_id", mtch.bid_id);
                        let _ = dict.set_item("ask_id", mtch.ask_id);
                        let _ = dict.set_item("bid_actor", "");
                        let _ = dict.set_item("ask_actor", "");
                        let _ = dict.set_item("bid_cluster", 0);
                        let _ = dict.set_item("ask_cluster", 0);
                        let _ = dict.set_item("energy", mtch.energy_kwh);
                        let _ = dict.set_item("price", mtch.price_euro_per_kwh);
                        let _ = dict.set_item("included_grid_fee", 0);
                        dict
                    }),
                );

                matches_dict.set_item("matches", matches_list)?;
                Ok(matches_dict.into())
            }

            CliResultOk::PubKeysBase58 {
                pubkeys_sr25519,
                pubkeys_ed25519,
            } => {
                let public_keys_dict = PyDict::new(py);

                match pubkeys_sr25519 {
                    Some(values) => {
                        if values.len() == 1 {
                            public_keys_dict.set_item("trusted_account", values[0].as_str())?;
                        } else {
                            let keys_list = PyList::new(py, values);
                            public_keys_dict.set_item("trusted_account", keys_list)?;
                        }
                    }
                    None => {
                        public_keys_dict.set_item("trusted_account", None::<&str>)?;
                    }
                }

                Ok(public_keys_dict.into())
            }

            CliResultOk::PayAsBidOutput(res) => match res {
                Some(vec) => {
                    let dict = PyDict::new(py);
                    let _ = dict.set_item("pay_as_bid_output", vec);
                    Ok(dict.into())
                }
                None => {
                    let py_dict = PyDict::new(py);
                    Ok(py_dict.into())
                }
            },

            CliResultOk::PayAsBidProofOutput(res) => {
                let dict = PyDict::new(py);

                println!("{:?}", res);

                dict.set_item("output", {}).unwrap();

                Ok(dict.into())
            }

            _ => {
                let error_message = "An unexpected error occurred.";
                let py_err = PyErr::new::<exceptions::PyException, _>(error_message);
                Err(py_err)
            }
        },
        Err(cli_error) => {
            let error_message = format!("An error occurred: {:?}", cli_error);
            let py_err = PyErr::new::<exceptions::PyException, _>(error_message);
            Err(py_err)
        }
    }
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
