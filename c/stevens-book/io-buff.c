#include "apue.h"

int
main(void)
{
    int c;
    int fd = 1;
    FILE *fp;

    if ((fp = fdopen(fd, "r+")) == NULL)
        printf("file descripter %i is not found\n", fd);
    while ((c = getc(stdin)) != EOF)
        if (putc(c, fp) == EOF)
            printf("output error\n");
        else
            sleep(1);
    if (ferror(stdin))
        printf("input error\n");
    exit(0);
}
