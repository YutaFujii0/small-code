#include "apue.h"

int
main(void)
{
    int n;
    int fd[2];
    pid_t pid;
    char line[MAXLINE];

    if (pipe(fd) < 0)
        printf("pipe error\n");
    if ((pid = fork()) < 0) {
        printf("fork error\n");
    } else if (pid == 0) { /* child */
        close(fd[1]);
        n = read(fd[0], line, MAXLINE);
        write(STDOUT_FILENO, line, n);
    } else { /* parent */
        close(fd[0]);
        write(fd[1], "hello world\n", 12);
    }
    exit(0);
}
