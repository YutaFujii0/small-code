#include "apue.h"
#include <sys/wait.h>

char *env_init[] = { "USER=unknown", "PATH=/tmp", NULL };

int
main(void)
{
    pid_t pid;

    if ((pid = fork()) < 0) {
        printf("fork error\n");
    } else if (pid == 0) {
        if (execle("./echoall", "echoall", "myarg1", "MY arg2", (char *)0, env_init) < 0)
            printf("execle error\n");
    }

    if (waitpid(pid, NULL, 0) < 0)
        printf("wait error\n");

    if ((pid = fork()) < 0) {
        printf("fork error\n");
    } else if (pid == 0) {
        if (execlp("echoall", "echoall", "only 1 arg", (char *)0) < 0)
            printf("execlp error\n");
    }

    if (waitpid(pid, NULL, 0) < 0)
        printf("wait error\n");

    exit(0);
}
