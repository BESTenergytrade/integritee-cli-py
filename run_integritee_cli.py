import argparse
import integritee_cli_py

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Rust CLI with different commands")
    parser.add_argument(
        "--command",
        type=str,
        required=True,
        choices=[
            "new_account_cmd",
            "new_trusted_account_cmd",
            "pay_as_bid_cmd",
            "get_market_results_cmd",
            "pay_as_bid_proof_cmd",
            "verify_proof_cmd",
        ],
        help="Specify the command to run",
    )

    parser.add_argument(
        "--params",
        nargs="+",  # Allows multiple arguments for params
        required=True,  # Make the params argument required
        help="Parameters for the command",
    )

    args = parser.parse_args()

    node_url = "ws://127.0.0.1"
    node_port = "9944"
    worker_url = "wss://127.0.0.1"
    trusted_worker_port = "2000"

    try:
        command_name = args.command
        params = args.params
        integritee_cli_py.run_cli(
            node_url, node_port, worker_url, trusted_worker_port, command_name, params
        )
    except Exception as e:
        print("Encountered an error:", e)
