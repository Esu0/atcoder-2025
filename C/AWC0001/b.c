#include<stdio.h>

int N,L,R;
int P[101];

int main(void) {
    scanf("%d %d %d", &N, &L, &R);
    for (int i = 1; i <= N; i++) {
        int p;
        scanf("%d", &p);
        if (P[p] == 0) P[p] = i;
    }
    int k = R;
    while (k >= L) {
        if (P[k] != 0) {
            printf("%d\n", P[k]);
            return 0;
        }
        k--;
    }
    printf("-1\n");
}