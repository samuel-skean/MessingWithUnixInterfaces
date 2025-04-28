#include <stdio.h>
#include <fcntl.h>
#include <signal.h>
#include <assert.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/stat.h>

// TODO: Try feature test macros for siginfo_t and its associated constants.
// Will those make VSCode's C/C++ extension happier?


// Notes:
//
// This program dies to SIGHUP when run with `nohup`, or logs the signal to a
// file if a filename is provided. It does these things when run without
// `nohup`, too.
//
// This includes (of course?) when it receives SIGHUP when a shell propogates a
// SIGHUP it receives, as `bash` does by default. The SIGHUP sent to the shell
// is often because an ssh session ended because of connection issues or
// unceremonious termination.
//
// IMO, the `disown -h` built-in is just better than `nohup` when you can do it
// - it prevents the propogation of the SIGHUP, while letting the process handle
// SIGHUP as it sees fit. Also, I think if `nohup` simply spawned a process
// running the program it was tasked to run as a child, making it a grandchild
// of the shell, that grandchild would be immune to the shell's propogation of
// SIGHUP (unless of course `nohup` propogated it, which would be silly). That's
// something to check!

int logfile_fd;
void sighup_sigaction_handler(int sig, siginfo_t *info, void *ucontext);

int main(int argc, char *argv[]) {

	sigset_t all_sigs;
	sigfillset(&all_sigs);

	struct sigaction sighup_act = {
		.sa_flags = 0,
		.sa_mask = all_sigs,
	};

	if (argc == 1) {
		printf("No filename provided as an argument. Will die to SIGHUP.\n");

		sighup_act.sa_handler = SIG_DFL;
	} else if (argc == 2) {
		char * logfile_name = argv[1];
		printf("Opening file %s as the log file.\n", logfile_name);

		// TODO: Stop modifying the umask, just let it be.
		//
		// Gotta set the umask to allow the open to create the file with the permissions we want!
		// A bit turned *on* in the mask is a bit open can*not* turn on!
		umask(0111); // I don't want this program making executable files.
		logfile_fd = open(logfile_name, O_WRONLY | O_CREAT | O_APPEND, 0666);

		if (logfile_fd < 0) {
			perror("Opening the log file failed with error: ");
			exit(1);
		}

		sighup_act.sa_flags |= SA_SIGINFO;
		sighup_act.sa_sigaction = sighup_sigaction_handler;

	} else {
		fprintf(stderr, "Invalid number of arguments: %d.\nMust be invoked with 1 or 2 arguments (including the name of the program).", argc);
		exit(1);
	}

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

void sighup_sigaction_handler(int sig, siginfo_t *info, void *ucontext) {
	(void)ucontext; // Suppress usage warning.

	assert(sig == info->si_signo);

	// Not at all guaranteed, but would be interesting to know if it changes:
	assert(info->si_errno == 0);
	// FIXME: Bad to do any stdio in the signal handler:
	FILE* logfile = fdopen(logfile_fd, "w");

	// TODO: What does this manpage mean when it describes how si_code is set
	// for a ptrace event? It describes si_code as an int - but then seems to
	// say the ptrace event will be in the high byte and that it's value shifted
	// left by 8 bits is OR'd with SIGTRAP to get si_code. Shifting something 8
	// bits to the left move it 1 byte to the left, and `int` is very commonly 4
	// bytes - I would assume it is. Are the `PTRACE_EVENT_*` values such that
	// only their 3rd least-significant byte (3rd from the right) is meaningful?
	// I'm sure this has a good rationale, since this same kind of C expression
	// involving ptrace events and a leftward 8-bit bitshift is used other
	// places in the man pages, sometimes with reference to "the higher byte" of
	// something, too.
	//
	// https://man7.org/linux/man-pages/man2/sigaction.2.html (search for "The si_code field")

	// siginfo_t may be a union, but these should work regardless of signal.
	if (info->si_code == SI_USER || info->si_code == SI_QUEUE) {
		fprintf(logfile, "SIGNO: %d si_code: %d Sender's PID: %d Sender's real UID: %d\n", info->si_signo, info->si_code, info->si_pid, info->si_uid);
	} else {
		fprintf(logfile, "SIGNO: %d si_code: %d\n", info->si_signo, info->si_code);
	}

	// FIXME: Causes issues the second time around. Appears to cause a segfault.
	fclose(logfile);
}
