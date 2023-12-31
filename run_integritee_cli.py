import os
from dotenv import load_dotenv
import argparse
import sys
import integritee_cli_py


# Load environment variables from .env file
load_dotenv()

# Access environment variables
NODE_URL = os.getenv("NODE_URL")
NODE_PORT = os.getenv("NODE_PORT")
WORKER_URL = os.getenv("WORKER_URL")
TRUSTED_WORKER_PORT = os.getenv("TRUSTED_WORKER_PORT")

# Mapping commands to their required parameter names
COMMAND_PARAMETER_INFO = {
    "new_account_cmd": ([], 0),
    "new_trusted_account_cmd": (
        [
            "mrenclave",
        ],
        1,
    ),
    "pay_as_bid_cmd": (["mrenclave", "account", "orders_string"], 3),
    "get_market_results_cmd": (["mrenclave", "account", "timestamp"], 3),
    "pay_as_bid_proof_cmd": (["mrenclave", "account", "timestamp", "actor_id"], 4),
    "verify_proof_cmd": (["mrenclave", "account", "merkle_proof_json"], 3),
}


def validate_parameters(command, params):
    required_params, required_count = COMMAND_PARAMETER_INFO.get(command, ([], 0))
    if len(params) != required_count:
        missing_params = required_count - len(params)
        param_names = ", ".join(required_params)
        raise argparse.ArgumentError(
            None,
            f"{command} requires {missing_params} more parameter(s): {param_names}",
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Rust CLI with specific commands")
    parser.add_argument(
        "--command",
        type=str,
        choices=COMMAND_PARAMETER_INFO.keys(),
        help="Please specify the command to run",
    )

    parser.add_argument(
        "--params",
        nargs="+",
        default=[],
        help="Parameters for the command",
    )

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(1)

    try:
        validate_parameters(args.command, args.params)

    except argparse.ArgumentError as e:
        print(str(e))
        parser.print_help()
        sys.exit(1)

    try:
        command_name = args.command
        params = args.params
        integritee_cli_py.run_cli(
            NODE_URL, NODE_PORT, WORKER_URL, TRUSTED_WORKER_PORT, command_name, params
        )
    except Exception as e:
        print("Encountered an error:", e)
