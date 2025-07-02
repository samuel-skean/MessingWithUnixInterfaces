#!/usr/bin/env python3
import sys

print(f"Here are my arguments as reported by sys.argv: {sys.argv}.")
print(
    "On Linux, this prints the name used to invoke me when invoked with `python3 <name>`. But when invoked directly, with `<name>`, it prints the concatenation of the entry in the PATH used to find it with the name passed on the command line as the zeroth argument."
)
print(f"The precise nature of this is documented here: https://docs.python.org/3/library/sys.html#sys.argv")
