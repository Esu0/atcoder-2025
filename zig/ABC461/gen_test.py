import random
N = 100
Q = 10000
print(f"{N} {Q}")
for _ in range(Q):
    t = random.randint(1, 2)
    x = random.randint(1, N)
    print(f"{t} {x}")
