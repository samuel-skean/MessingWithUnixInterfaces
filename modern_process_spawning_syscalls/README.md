NOTE: All testing in this directory was done on Linux.
This is somewhat silly. Just two programs that I whipped up and ran each under `strace -e fork,vfork,clone,clone3` to produce this on standard error:
```
clone(child_stack=0xffffacf2c000, flags=CLONE_VM|CLONE_VFORK|SIGCHLD) = 27851
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=27851, si_uid=1000, si_status=0, si_utime=0, si_stime=0} ---
+++ exited with 0 +++
```
(The address varies between the two programs, and the pid varies between executions.)
Well that's neat!
Looks like `CLONE_VM` is responsible for not cloning the VM (\*sigh\*) and `CLONE_VFORK` is responsible for pausing the parent until (normally) the `execve`. Also, I think a call to `vfork` boils down to this same call.
https://man7.org/linux/man-pages/man2/clone.2.html
