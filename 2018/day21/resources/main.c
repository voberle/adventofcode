#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    long long r0 = atoi(argv[1]);

    long long r1 = 0;
    long long r4 = 0;

    while (1) {
        r4 = r1 | 0x10000;
        r1 = 16298264;

        while (1) {
            r1 = r1 + (r4 & 0xFF);
            r1 = r1 & 0xFFFFFF;
            r1 = r1 * 65899;
            r1 = r1 & 0xFFFFFF;

            if (r4 < 256) {
                goto line28;
            }

            r4 = r4 / 256;
        }

    line28:
        if (r1 == r0) {
            break;
        }
    }
    printf("r0 %lli r1 %lli r4 %lli\n", r0, r1, r4);

    return 0;
}
