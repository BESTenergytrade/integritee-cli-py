import integritee_cli_py

node_url = "ws://127.0.0.1"
node_port = "9944"
worker_url = "wss://127.0.0.1"
trusted_worker_port = "2000"

if __name__ == "__main__":
    integritee_cli_py.run_cli(node_url, node_port, worker_url, trusted_worker_port, "balanced")
    print("TERMINATE")

