# Integritee Python Client

Use Pyo3 Lib to wrap the integritee-cli into a python lib.

```
python -m venv venv/
pip install maturin
```

To install the lib to your current python environment use:
```
maturin develop
```

## Troubleshooting

* **wasmtime**: error[E0463]: can't find crate for `std`
```
rustup target add --toolchain nightly wasm32-unknown-unknown
```
