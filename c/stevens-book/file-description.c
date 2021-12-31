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
