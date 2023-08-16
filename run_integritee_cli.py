import argparse
import sys
import integritee_cli_py

# Mapping commands to their required parameter names
COMMAND_PARAMETER_NAMES = {
    "new_account_cmd": [],
    "new_trusted_account_cmd": [
        "mrenclave",
    ],
    "pay_as_bid_cmd": ["mrenclave", "orders_string"],
    "get_market_results_cmd": ["mrenclave", "timestamp"],
    "pay_as_bid_proof_cmd": ["mrenclave", "timestamp", "actor_id"],
    "verify_proof_cmd": ["verify_proof_cmd", "merkle_proof_json"],
}


def display_help(parser, error_message=None):
    if error_message:
        print(error_message)
    parser.print_help()
    sys.exit(1)


def validate_required_params(command, params):
    required_parameter_names = COMMAND_PARAMETER_NAMES.get(command, [])
    missing_parameter_names = [
        param_name
        for param_name in required_parameter_names
        if param_name not in params
    ]
    if missing_parameter_names:
        missing_params_str = ", ".join(missing_parameter_names)
        error_message = f"Parameters ({missing_params_str}) are required for the '{command}' command"
        raise argparse.ArgumentError(None, error_message)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Rust CLI with specific commands")
    parser.add_argument(
        "--command",
        type=str,
        required=True,
        choices=COMMAND_PARAMETER_NAMES.keys(),
        help="Please specify the command to run",
    )

    parser.add_argument(
        "--params",
        nargs="+",  # Allows multiple arguments for params
        default=[],
        help="Parameters for the command",
    )

    args = parser.parse_args()

    if not args.command:
        display_help(parser, "error: the following arguments are required: --command")

    try:
        validate_required_params(args.command, args.params)
    except argparse.ArgumentError as e:
        display_help(parser, str(e))

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
