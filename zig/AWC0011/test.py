import random

N = 1000
Q = 1000
V = [0]*N
P = [0]*(N-1)
for i in range(N):
    V[i] = random.randint(1, 1000000000)

for i in range(N-1):
    P[i] = random.randint(1, i+1)

print(f"{N} {Q}")
print(" ".join(map(str, V)))
print(" ".join(map(str, P)))
for _ in range(Q):
    print(f"{random.randint(1, N)}")
