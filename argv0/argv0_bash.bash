#!/usr/bin/env bash
# TODONOW: Print a nice message.
echo "$0"
echo $'On Linux, this prints the name used to invoke me when invoked with `bash <name>`. But when invoked directly, with `<name>`, it prints the concatenation of the entry in the PATH used to find it with the name passed on the command line as the zeroth argument. If no PATH entry was used to find me, it just prints the name passed on the command line as the zeroth argument.'
