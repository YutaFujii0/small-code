#include <stdio.h>

int main() {
    int mynumber = 0;
    printf("Hello World %i\n", mynumber);

    // pointer
    int y = 7;
    int *p = &y;
    printf("%p, %i\n", p, *p);

    // pointer index
    int x[5] = {1,2,3,4,5};
    int *p2 = x;
    p2[0] = 100;
    *(p2+2) = 200;
    printf("x[0] = %i\n", x[0]);
    printf("x[1] = %i\n", x[1]);
    printf("x[2] = %i\n", x[2]);
    printf("x[3] = %i\n", x[3]);
    printf("x[4] = %i\n", x[4]);

    // loop
    for (int counter=0; counter < 5; counter++){
        printf("for loop %i\n", x[counter]);
    }

    int do_count = 0;
    do {
        printf("This is inside do loop\n");
        do_count++;
    } while (do_count < 3);
}

// # ... preprocessor
