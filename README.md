# Integritee Python Client

Use `Pyo3` Lib to wrap the `integritee-cli` into a Python lib.

## Table of Contents

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [Updating Git Submodules](#updating-git-submodules)
  - [Creating and Activating venv](#creating-and-activating-venv)
  - [Managing Environment Variables](#managing-environment-variables)
  - [Installing Dependencies](#installing-dependencies)
- [Development](#development)
  - [Using maturin](#using-maturin)
- [Usage](#usage)
  - [Running the Client](#running-the-client)
  - [Available Commands](#available-commands)
- [Troubleshooting](#troubleshooting)

## Getting Started

Get started with the project setup and initial configuration.

## Prerequisites

In order to use the features, the nightly version of the compiler has
to be setup beforehand:

```
$ rustup override set nightly
```

## Installation

### Updating Git Submodules

`Cargo.toml` states the relative dependencies at the `shared` folder.
The submodules have to be initialized and updated before building:

```
$ git submodule update --init --recursive
```

### Creating and Activating venv

Install `integritee_cli_py` as lib into virtual environment:

```
$ python -m venv venv/
$ source venv/bin/activate
# unset conda_prefix - if existing - for maturin to work
$ conda deactivate
```

### Managing Environment Variables

Please follow these steps to set up and use environment variables:

1. Copy the `.env.example` file and rename it to `.env`:

```bash
$ cp .env.example .env
```

2. Open the `.env` file and provide values for the environment variables.

### Installing Dependencies

Install the necessary dependencies using the provided `requirements.txt` file:

```bash
$ pip install -r requirements.txt
```

## Development

### Using maturin

To install the lib to your current python environment use:

```
$ maturin develop
```

## Usage

Here's how you can make the most of our project!

### Run as Command Line Tool

Test import and execution of `integritee_cli_py` in python:

```
$ python run_integritee_cli.py --command <COMMAND-NAME> --params <PARAMETER-NAME>
```

### Available Commands

The `integritee-cli-py` provides the following CLI commands:

#### Get help:

**Syntax:**

```bash
$ python3 run_integritee_cli.py --help
```

**Output:**

```bash

usage: run_integritee_cli.py [-h] [--command {new_account_cmd,new_trusted_account_cmd,pay_as_bid_cmd,get_market_results_cmd,pay_as_bid_proof_cmd,verify_proof_cmd}] [--params PARAMS [PARAMS ...]]

Run Rust CLI with specific commands

options:
  -h, --help            show this help message and exit
  --command {new_account_cmd,new_trusted_account_cmd,pay_as_bid_cmd,get_market_results_cmd,pay_as_bid_proof_cmd,verify_proof_cmd}
                        Please specify the command to run
  --params PARAMS [PARAMS ...]
                        Parameters for the command
```

#### Create a new account:

**Syntax:**

```bash
$ python3 run_integritee_cli.py --command new_account_cmd
```

#### Create a new trusted account:

**Syntax:**

```bash
$ python3 run_integritee_cli.py --command new_trusted_account_cmd --params <MRENCLAVE>
```

#### Pay As Bid:

**Syntax:**

```bash
$ python3 run_integritee_cli.py --command pay_as_bid_cmd --params <MRENCLAVE> <ACCOUNT> <ORDERS_STRING>
```

**Example:**

```bash
$ python3 run_integritee_cli.py --command pay_as_bid_cmd --params 9PPeGELLdD9Uw1mVJbUGTeRpGzPBGb1bdEk6TCL4pPCE 5Dsni69ozXZZwpxyCGjLq8KQnBpGrtPnbykepgst2Tbh7NuY "[{\"id\":0,\"order_type\":\"ask\",\"time_slot\":\"2022-10-04T05:06:07+00:00\",\"actor_id\":\"actor_0\",\"cluster_index\":0,\"energy_kwh\":5,\"price_euro_per_kwh\":0.19},{\"id\":1,\"order_type\":\"bid\",\"time_slot\":\"2022-10-04T05:06:07+00:00\",\"actor_id\":\"actor_1\",\"cluster_index\":0,\"energy_kwh\":8.8,\"price_euro_per_kwh\":0.23}]"
```

#### Get Market Results:

**Syntax:**

```bash
$ python3 run_integritee_cli.py --command get_market_results_cmd --params <MRENCLAVE> <ACCOUNT> <TIMESTAMP>
```

**Example:**

```bash
$ python3 run_integritee_cli.py --command get_market_results_cmd --params 9PPeGELLdD9Uw1mVJbUGTeRpGzPBGb1bdEk6TCL4pPCE 5Dsni69ozXZZwpxyCGjLq8KQnBpGrtPnbykepgst2Tbh7NuY 2022-10-04T05:06:07+00:00
```

#### Get Bid Proof:

**Syntax:**

```bash
$ python3 run_integritee_cli.py --command pay_as_bid_proof_cmd --params <MRENCLAVE> <ACCOUNT> <TIMESTAMP> <ACTOR_ID>
```

**Example:**

```bash
$ python3 run_integritee_cli.py --command pay_as_bid_proof_cmd --params 9PPeGELLdD9Uw1mVJbUGTeRpGzPBGb1bdEk6TCL4pPCE 5Dsni69ozXZZwpxyCGjLq8KQnBpGrtPnbykepgst2Tbh7NuY 2022-10-04T05:06:07+00:00 actor_0
```

#### Verify Proof:

**Syntax:**

```bash
$ python3 run_integritee_cli.py --command verify_proof_cmd --params <MRENCLAVE> <ACCOUNT> <MERKLE_PROOF_JSON>
```

**Example:**

```bash
python3 run_integritee_cli.py --command verify_proof_cmd --params 9PPeGELLdD9Uw1mVJbUGTeRpGzPBGb1bdEk6TCL4pPCE 5Dsni69ozXZZwpxyCGjLq8KQnBpGrtPnbykepgst2Tbh7NuY "{\"root\":\"0xeae9131721b25db95622605c99a62f56f2e3b47e7f54f9b0653055b11b8d37b8\",\"proof\":[\"0xf147000d21d56a303b688da5bf1294d865518a8fb889af48ca21d12af5a6d823\"],\"number_of_leaves\":2,\"leaf_index\":0,\"leaf\":[0,0,0,0,0,0,0,0,1,100,50,48,50,50,45,49,48,45,48,52,84,48,53,58,48,54,58,48,55,43,48,48,58,48,48,28,97,99,116,111,114,95,48,1,0,0,0,0,0,0,0,0,0,0,20,64,82,184,30,133,235,81,200,63]}"
```

## Troubleshooting

- **wasmtime**: error[E0463]: can't find crate for `std`

```
rustup target add --toolchain nightly wasm32-unknown-unknown
```
