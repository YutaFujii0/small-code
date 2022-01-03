#include "apue.h"
#include <errno.h>

#define BUFFSIZE 4096

static void
sig_hup(int signo)
{
    printf("SIGHUP received, pid = %ld\n", (long)getpid());
}

static void
pr_ids(char *name)
{
    printf("%s: pid = %ld, ppid = %ld, pgrp = %ld, tpgrp = %ld\n",
        name, (long)getpid(), (long)getppid(), (long)getpgrp(),
        (long)tcgetpgrp(STDIN_FILENO));
    fflush(stdout);
}

int
main(void)
{
    pid_t pid;
    char c;
    char buf[BUFFSIZE] = "hello world";

    pr_ids("parent");
    if ((pid = fork()) < 0) {
        printf("fork error\n");
    } else if (pid > 0) {
        sleep(5);
    } else {
        pr_ids("child");
        signal(SIGHUP, sig_hup);
        kill(getpid(), SIGTSTP);
        pr_ids("child");
        if (read(STDIN_FILENO, &c, 1) != 1)
            printf("read error %d on controlling TTY\n", errno);
        if (write(STDOUT_FILENO, buf, 11) != 11)
            printf("write error %d on controlling TTY\n", errno);
    }
}
