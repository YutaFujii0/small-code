#include "apue.h"

int
main(void)
{
    pid_t pid;
    pid_t pid2;

    printf("before fork\n");
    if ((pid = fork()) < 0) {
        printf("fork error\n");
    } else if (pid == 0) {
        printf("hello from child process\n");
        printf("pid = %ld, ppid = %ld\n", (long)getpid(), (long)getppid());
        exit(0);
    } else {
        sleep(3);
        printf("hello from parent process\n");
    }

    if ((pid2 = fork()) < 0) {
        printf("fork error\n");
    } else if (pid2 == 0) {
        sleep(3);
        printf("hello from child process-2\n");
        printf("pid = %ld, ppid = %ld\n", (long)getpid(), (long)getppid());
    } else {
        printf("hello from parent process-2\n");
    }
    exit(0);
}
