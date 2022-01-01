// stevens book ch 4.2
#include "apue.h"

int
main(int argc, char *argv[])
{
    struct stat buf;
    if (argc != 2) {
        printf("usage: ./file-description production.log\n");
        exit(0);
    }
    if (lstat(argv[1], &buf) < 0) {
        printf("lstat error\n");
        exit(0);
    } else {
        printf("stat result:\n");
        printf("mode    %hu\n", buf.st_mode);
        printf("ino     %llu\n", buf.st_ino);
        printf("dev     %d\n", buf.st_dev);
        printf("rdev    %d\n", buf.st_rdev);
        printf("nlink   %hu\n", buf.st_nlink);
        printf("uid     %u\n", buf.st_uid);
        printf("gid     %u\n", buf.st_gid);
        printf("size    %lld\n", buf.st_size);
        // printf("%hu\n", buf.st_atim);
        // printf("%hu\n", buf.st_mtim);
        // printf("%hu\n", buf.st_ctim);
        printf("blksize %d\n", buf.st_blksize);
        printf("blocks  %lld\n", buf.st_blocks);
    }

    exit(0);
}

/*

`rm` command is referred to as unlink, not remove:
see how `nlink` value changes through touch, link, rm commands:

$ touch tmp2
$ ./file-description tmp2
stat result:
mode    33188
ino     10558326
dev     16777220
rdev    0
nlink   1
uid     503
gid     20
size    0
blksize 4096
blocks  0

$ link tmp2 tmp

$ ./file-description tmp
stat result:
mode    33188
ino     10558326
dev     16777220
rdev    0
nlink   2
uid     503
gid     20
size    0
blksize 4096
blocks  0

$ ./file-description tmp2
stat result:
mode    33188
ino     10558326
dev     16777220
rdev    0
nlink   2
uid     503
gid     20
size    0
blksize 4096
blocks  0

$ rm tmp2

$ ./file-description tmp
stat result:
mode    33188
ino     10558326
dev     16777220
rdev    0
nlink   1
uid     503
gid     20
size    0
blksize 4096
blocks  0

*/
