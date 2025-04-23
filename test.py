from rust_cache import rust_cache
import time

@rust_cache
def heavy_function(x):
    return x * x

for _ in range(10_000_000):
    heavy_function(10)