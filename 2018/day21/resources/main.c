#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    long long r0 = atoi(argv[1]);
    printf("r0 %lli\n", r0);

    long long r1 = 0;
    // R2 is IP
    long long r3 = 0;
    long long r4 = 0;

line6:
    // 6    bori 1 65536 4         r4 = r1 | 0x10000
    // 7    seti 16298264 8 1      r1 = 16298264
    r4 = r1 | 0x10000;
    r1 = 16298264;
line8:
    // 8    bani 4 255 5           r5 = r4 & 0xFF
    // 9    addr 1 5 1             r1 = r1 + r5
    // 10   bani 1 16777215 1      r1 = r1 & 0xFFFFFF
    // 11   muli 1 65899 1         r1 = r1 * 65899
    // 12   bani 1 16777215 1      r1 = r1 & 0xFFFFFF
    r1 = r1 + (r4 & 0xFF);
    r1 = r1 & 0xFFFFFF;
    r1 = r1 * 65899;
    r1 = r1 & 0xFFFFFF;

    // 13   gtir 256 4 5           r5 = 256 > r4 ? 1 : 0
    // 14   addr 5 2 2             R2 = R2 + r5, JUMP: if r5 == 1, skip next line
    // 15   addi 2 1 2             R2 = R2 + 1, JUMP: skip next line
    // 16   seti 27 1 2            R2 = 27, JUMP line 28
    // 17   seti 0 3 5             r5 = 0
    if (r4 < 256) {
        goto line28;
    }

line26:
    // 26   setr 5 3 4             r4 = r5   
    // 27   seti 7 7 2             R2 = 7, JUMP line 8
    r4 = r4 / 256;
    goto line8;

line28:
    // 28   eqrr 1 0 5             r5 = r1 == r0 ? 1 : 0
    // 29   addr 5 2 2             R2 = R2 + r5, JUMP: if r5 == 1, skip next line, ie program END
    // 30   seti 5 3 2             R2 = 5, JUMP line 6
    if (r1 != r0) {
        //printf("[%lli, %lli, IP, %lli, %lli, %lli, \n", r0, r1, r3, r4, r5);
        goto line6;
    }

    return 0;
}
