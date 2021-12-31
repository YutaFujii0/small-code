#include "apue.h"
#include <fcntl.h>

int
main(int argc, char *argv[])
{
    if (argc != 2)
        printf("usage: file-priviledge <filename>\n");
    if (access(argv[1], R_OK) < 0)
        printf("access denied for %s\n", argv[1]);
    else
        printf("read access OK\n");

    if (open(argv[1], O_RDONLY) < 0)
        printf("file open error for %s\n", argv[1]);
    else
        printf("open for reading OK\n");
    exit(0);
}

/* Example usage

$ ./file-priviledge /private/etc/master.passwd
access denied for /private/etc/master.passwd
file open error for /private/etc/master.passwd

$ sudo ./file-priviledge /private/etc/master.passwd
read access OK
open for reading OK

$ sudo chown root file-priviledge

$ ./file-priviledge /private/etc/master.passwd
access denied for /private/etc/master.passwd
file open error for /private/etc/master.passwd

$ sudo chmod u+s file-priviledge

$ ./file-priviledge /private/etc/master.passwd
access denied for /private/etc/master.passwd
open for reading OK

*/
