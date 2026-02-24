#include<stdlib.h>
#include<stdio.h>

int N,K;
int D[2<<17];
int compare(const void *pa, const void *pb) {
    return *(int *)pa - *(int *)pb;
}

int main(void) {
    scanf("%d %d", &N, &K);
    for (int i=0;i<N;i++)scanf("%d", D+i);
    qsort(D, N, sizeof(int), compare);
    long long sum = 0;
    for (int i=0;i<N-K;i++)sum += D[i];
    printf("%lld\n", sum);
}