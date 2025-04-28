#include <stdio.h>
#include <fcntl.h>
#include <signal.h>
#include <assert.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>

// Notes:
//
// This program dies to SIGHUP when run with `nohup`.
//
// This includes (of course?) when it receives SIGHUP when a shell
// propogates a SIGHUP it receives, as `bash` does by default. The SIGHUP
// sent to the shell is often because an ssh session ended because of
// connection issues or unceremonious termination.
//
// IMO, the `disown -h` built-in is just better when you can do it - it
// prevents the propogation of the SIGHUP, while letting the process handle
// SIGHUP as it sees fit.
// Also, I think if `nohup` simply spawned a process running the program it
// was tasked to run as a child, making it a grandchild of the shell, that
// grandchild would be immune to the shell's propogation of SIGHUP (unless
// of course `nohup` propogated it, which would be silly). That's something
// to check!

int logfile_fd;
void sighub_sigaction_handler(int sig, siginfo_t *info, void *ucontext);
	
int main(int argc, char *argv[]) {
	// printf("This program registers a handler with SIGHUP that writes to the file named as its only argument.\n");
	char * logfile_name = argv[1];
	// logfile_fd = open(logfile_name, O_CREAT | O_APPEND);
	// if (logfile_fd < 0) {
	// 	perror("Opening the log file failed with error: ");
	// 	exit(1);
	// }
	
	sigset_t all_sigs;
	sigfillset(&all_sigs);

	const struct sigaction sighup_act = {
//		.sa_flags = SA_SIGINFO,
//		.sa_sigaction = sighup_sigaction_handler,
		.sa_handler = SIG_DFL,
		.sa_mask = all_sigs,
	};

	int result = 0;

	if ((result = sigaction(SIGHUP, &sighup_act, NULL))) {
		perror("sigaction() failed: ");
		exit(1);
	}

	while (1) {
		// I am close to having the race-condition described below associated with `pause`,
		// because I may receive the signal I'm interested in (`SIGHUP`) at any time,
		// including before `pause`. This would cause `pause` to pause until the *next*
		// receipt of a signal - I am not guaranteed to have control in this function after
		// every receipt of a signal.
		//
		// `sigsuspend` on its own would not help with this, but `sigprocmask` and `sigsuspend`
		// together would.
		//
		// But, my use case is actually one where the "main program"
		// (everything but the signal handlers) does not need to be aware of
		// the arrival of a signal, so who cares!
		//
		// https://www.gnu.org/savannah-checkouts/gnu/libc/manual/html_node/Pause-Problems.html#Pause-Problems
		assert(pause() == -1);
		assert(errno == EINTR);
	}
}

