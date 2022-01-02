#include "apue.h"

int globalvar = 6;
char buf[] = "You can immediately see this either file descripter 1 is stdout or a file because write is not buffered\n";

int
main(void)
{
    int var;
    pid_t pid;
    char buf2[] = "before fork: This is placed in main memory buffer. If stdout is terminal, you'll see this once since terminal adopt line buffer, flushing right after printf called. If stdout is a file, this sentence is buffered both parent and child process and flushed after all, causing to write twice.\n";

    var = 88;
    if (write(STDOUT_FILENO, buf, sizeof(buf)-1) != sizeof(buf) - 1)
        printf("write error\n");
    sleep(5);

    printf("%s", buf2);
    if ((pid = fork()) < 0) {
        printf("fork error\n");
    } else if (pid == 0) {
        globalvar++; /* child */
        var++;
    } else {
        sleep(2); /* parent */
    }
    printf("pid = %ld, glob = %d, var = %d\n", (long)getpid(), globalvar, var);
    exit(0);
}
