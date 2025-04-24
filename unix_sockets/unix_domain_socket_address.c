#include <sys/socket.h>
#include <sys/un.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    // I only comment what I don't understand now.

    int recving_sd = socket(AF_UNIX, SOCK_DGRAM, 0 /* default protocol, good for most domains */);
    if (recving_sd < 0) {
        perror("socket() for recving_sd failed");
        exit(1);
    }
    
    struct sockaddr_un recving_addr = {
        .sun_family = AF_UNIX,
        .sun_path = "recving.sock",
    };

    if (bind(recving_sd, (struct sockaddr *) &recving_addr, 20) < 0) {
        perror("bind() for recving_sd failed");
        exit(1);
    }

    struct sockaddr_un sending_addr = {
        .sun_family = AF_UNIX,
        .sun_path = "sending.sock",
    };

    int sending_sd = socket(AF_UNIX, SOCK_DGRAM, 0);

    if (sending_sd < 0) {
        perror("socket() for sending_sd failed");
        exit(1);
    }

    // TODO: I still haven't figured out if connect fundamentally changes what
    // you can do with a Unix Datagram socket.
    
    // sendto(sending_sd, "Hello", 5, 0, &recving_sd, );




}