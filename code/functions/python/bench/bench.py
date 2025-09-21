"""
Provides a minimal benchmark harness to measure function performance via timestamps.
"""
import sys
import time


def timestamp() -> int:
    """
    Creates and returns a timestamp in microseconds since Unix Epoch.
    """
    return round(time.time_ns() / 1000)


def benchmark(function):
    """
    Runs the given function, recording timestamps before and after execution 
    and logging them to stderr.
    """
    start = timestamp()
    function()
    end = timestamp()
    print(f"{start} {end}", file=sys.stderr)  # for logfile
