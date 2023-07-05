use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import the RPC client
use clap::Parser;
use integritee_cli::{commands, Cli};

#[pymodule]
fn integritee_cli_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}

#[pyfunction]
fn run_cli(node_url: String, node_port: String, worker_url: String, trusted_worker_port: String, command: String) -> PyResult<()> {
    println!("Run Cli");
    let cli = Cli::try_parse_from(vec![command, "abc".to_string()]);
    cli.unwrap();
    /* Alternative:
    let cli = Cli::parse();
    // Run the CLI command
    //println!("Execute Command");
    commands::match_command(&cli).unwrap();
    */
    Ok(())
}

