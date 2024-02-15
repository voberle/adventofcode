# Python version of the assembly that computes the input (after line 17).

r0 = 1

r1 = 0
r2 = 0
r3 = 0
r5 = 0

r1 = r1 + 2
r1 = r1 * r1
r4 = 19
r1 = r4 * r1
r1 = r1 * 11
r5 = r5 + 1
r4 = 22
r5 = r4 * r5
r5 = r5 + 2
r1 = r1 + r5

if r0 == 0:
    print("r1", r1)
    quit()

r4 = 27
r5 = r4
r4 = 28
r5 = r4 * r5
r4 = 29
r5 = r4 + r5
r4 = 30
r5 = r4 * r5
r5 = r5 * 14
r4 = 32
r5 = r5 * r4
r1 = r1 + r5

print("r1", r1)
