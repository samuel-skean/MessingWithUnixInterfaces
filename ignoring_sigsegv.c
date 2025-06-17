#include <signal.h>
#include <unistd.h>
#include <assert.h>

// CONSIDER: What happens when you actually SIG_IGN SIGSEGV?
// Whether you block or you ignore sigsegv, the kernel kills you if you get it from a page fault.
void handle_sigint(int signum) {
    assert(signum == SIGINT);
    static const char msg[] = "Handling SIGINT, after raising SIGINT.";

    write(1, msg, sizeof(msg));
    raise(SIGINT);

    write(1, msg, sizeof(msg));
}

void handle_sigsegv(int signum) {
    assert(signum == SIGSEGV);
    static const char msg[] = "Handling SIGSEGV.";
    write(1, msg, sizeof(msg));
}

int main(void) {
    sigset_t all_signals;
    sigfillset(&all_signals);

    sigset_t no_signals;
    sigemptyset(&no_signals);

    struct sigaction sigint_sa = {
        .sa_handler = handle_sigint,
        .sa_mask = no_signals,
        .sa_flags = SA_NODEFER,
    };
    sigaction(SIGINT, &sigint_sa, NULL);


    struct sigaction sigsegv_sa = {
        .sa_handler = handle_sigsegv, // Eek!
        .sa_mask = no_signals,
        .sa_flags = 0,
    };
    // sigaction(SIGSEGV, &sigsegv_sa, NULL);

    while (1) {}
}
