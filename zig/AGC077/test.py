import random
R = 100
N = 29
T = (N - 1) * R
print(T)
for i in range(0, N - 1):
    for _ in range(R):
        print(N)
        S = ['A']*i + ['B'] * (N - 2 - i)
        random.shuffle(S)
        print(''.join(['A'] + S + ['B']))
        random.shuffle(S)
        print(''.join(['B'] + S + ['A']))
