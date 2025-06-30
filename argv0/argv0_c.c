#include <stdio.h>
#include <assert.h>

int main(int argc, char ** argv) {
    assert(argc >= 0);
    unsigned int unsigned_argc = (unsigned int) argc;
    puts("This is a C program.");
    puts("My arguments are:");
    for (unsigned int i = 0; i < unsigned_argc; i++) {
        printf("    %s\n", argv[i]);
    }
    puts("On Linux, the first item printed is the first argument on the command line used to run me.");
    puts("");
}
