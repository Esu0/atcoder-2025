import random
T = 10000
N = 10
print(T)
for _ in range(T):
    print(N)

    for _ in range(N):
        print(f"{random.randint(1, N)} {random.randint(1, N)} {random.randint(1, N)}")
