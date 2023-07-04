use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import the RPC client
use clap::Parser;
use integritee_cli::{commands, Cli};

#[pymodule]
fn integritee_rpc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}

#[pyfunction]
fn run_cli(node_url: String, node_port: String, worker_url: String, trusted_worker_port: String) -> PyResult<()> {

    let cli = Cli::parse();

    // Run the CLI command
    commands::match_command(&cli).unwrap();
    Ok(())
}

