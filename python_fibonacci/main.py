import time

import rust_fibonacci

from . import py_fibonacci


def benchmark(n: int = 10):

    start_time = time.time()
    rust_value = rust_fibonacci.fibonacci(n)
    rust_elapsed_time = time.time() - start_time

    start_time = time.time()
    py_value = py_fibonacci.fibonacci(n)
    python_elapsed_time = time.time() - start_time

    print(f"Rust took {rust_elapsed_time:.4f} seconds to run and returned {rust_value}")
    print(f"Python took {python_elapsed_time:.4f} seconds to run and returned {py_value}")
