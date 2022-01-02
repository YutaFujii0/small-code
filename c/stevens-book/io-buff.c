#include "apue.h"
#include <fcntl.h>

int
main(void)
{
    int c;
    int fd;
    FILE *fp;

    if ((fd = open("tmp2", O_RDWR)) == -1) {
        printf("file open error\n");
        exit(0);
    }

    if ((fp = fdopen(fd, "r+")) == NULL)
        printf("file descripter 3 is not found\n");
    while ((c = getc(stdin)) != EOF)
        if (putc(c, fp) == EOF)
            printf("output error\n");
        else
            // putc(c, stdout);
            printf("%d\n", c);
            sleep(1);
    if (ferror(stdin))
        printf("input error\n");
    exit(0);
}
