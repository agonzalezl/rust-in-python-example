# Rust in Python example

This repository demonstrates how to use Rust code from Python with the help of Maturin, a tool for building and distributing Rust-based Python modules. As example it provides an simple and inefficient implementation of fibonacci algorithm.


## Project structure

In a `maturin` project, you can leave your Rust code in the `rust` and your Python code in the location that you prefer (remember to set it in pyproject.toml `tool.maturin.python-source`)

```
rust-in-python-example
├── python_fibonacci  # Your main python code
│   └── my_project
│       ├── __init__.py
│       ├── main.py
│       └── py_fibonacci.py
├── pyproject.toml
├── README.md
└── rust
    |── Cargo.toml
    └── src
        └── lib.rs  # Rust implementation
```

## How to try it

Clone this repo and install it with pip

```
pip install .
```
Then try the benchmark
```
from python_fibonacci.main import benchmark
benchmark(40)

> Rust took 0.3438 seconds to run and returned 102334155
> Python took 11.5431 seconds to run and returned 102334155
```
