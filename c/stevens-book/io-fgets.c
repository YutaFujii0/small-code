#include "apue.h"

int
main(void)
{
    char buf[MAXLINE];

    while (fgets(buf, MAXLINE, stdin) != NULL)
        if (fputs(buf, stdout) == EOF)
            printf("output error\n");
    if (ferror(stdin))
        printf("input error\n");
    exit(0);
}
