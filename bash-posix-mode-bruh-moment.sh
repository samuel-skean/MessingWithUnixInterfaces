#!/usr/bin/env -S bash --posix
# Run through bash not in posix mode to see an obviously unintended result.
# Relying on this behavior sure is weird, though.

alias lsl='ls -l'
lsl

# Lots of other bash POSIX-compatibility weirdnesses including with variable assignment before special builtins. That difference can be verified here in the GNU Bash manual: https://www.gnu.org/software/bash/manual/bash.html#Bash-POSIX-Mode and the default behavior in that case can be verified as a violation of POSIX by reading IEEE 1003.1-2024, page 2501, line 81257.
# The difference in *this* file is equally easy to find documented in the GNU Bash manual, but the behavior is less explicitly described in the POSIX manual IMO, because the relevant part (page 2477) doesn't discuss the interactivity of the shell.
