#include <stdio.h>
#include <fcntl.h>
#include <signal.h>
#include <assert.h>
#include <stdlib.h>

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

	sigset_t no_sigs;
	sigemptyset(&no_sigs);

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
		// Though I am not using pause, I still have the race-condition
		// problem with pause described here because signals could arrive
		// whenever:
		//
		// https://www.gnu.org/savannah-checkouts/gnu/libc/manual/html_node/Pause-Problems.html#Pause-Problems
		assert(sigsuspend(&no_sigs) == -1);
		perror("sigsuspend() interrupted by: ");
	}
}

