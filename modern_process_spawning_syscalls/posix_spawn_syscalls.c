#include <spawn.h>
#include <errno.h>
#include <stdio.h>

extern char **environ;

int main(void) {
    pid_t child_pid;
    int posix_spawn_error;
    char * argv[] =  { "/usr/bin/true", NULL };

    posix_spawnattr_t spawnattr;
    posix_spawnattr_init(&spawnattr);
    // It's not very obviously documented that `NULL` is a valid value for the file actions, but the man page mentions historical successful behavior with it set.
    posix_spawn_error = posix_spawn(&child_pid, "/usr/bin/true", NULL, &spawnattr, argv, environ);

    if (posix_spawn_error != 0) {
        errno = posix_spawn_error;
        perror("posix_spawn error");
    }

}
