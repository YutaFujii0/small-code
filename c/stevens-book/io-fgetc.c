#include "apue.h"

int
main(void)
{
    int c;
    while ((c = fgetc(stdin)) != EOF)
        if (fputc(c, stdout) == EOF)
            printf("output error");
    if (ferror(stdin))
        printf("input error");
    exit(0);
}
