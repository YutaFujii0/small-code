#include "apue.h"
#include <fcntl.h>

int
main(int argc, char *argv[])
{
    int fd;
    int i;

    if (argc < 2)
        printf("usage: file <filepath>\n");
    for (i = 1; i < argc; i++) {
        if ((fd = open(argv[i], O_RDONLY)) < 0)
            printf("read error\n");
        else
            printf("file descripter for %s is %i\n", argv[i], fd);
    }
    exit(0);
}
