use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Import the RPC client
use integritee_cli::{commands, Cli};

#[pymodule]
fn integritee_rpc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}

#[pyfunction]
fn run_cli(node_url: String, node_port: String, worker_url: String, trusted_worker_port: String) -> PyResult<()> {
    /*
    let cli = Cli {
        node_url,
        node_port,
        worker_url,
        trusted_worker_port,
        command: /* ... */,
    };
    */
    let cli = Cli::parse();

    // Run the CLI command
    commands::match_command(&cli) {
        Ok(_) => Ok(()),
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", err))),
    }
}

