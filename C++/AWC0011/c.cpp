#include<iostream>
using namespace std;

int main(void) {
    int N;
    unsigned long long K;
    cin >> N >> K;
    unsigned long long o = 0;
    int ans = 0;
    for (int i = 0; i < N; i++) {
        unsigned long long a;cin>>a;
        if ((a | K) == K) {
            ans++;
            o |= a;
        }
    }
    if (o != K) {
        ans = -1;
    }
    cout << ans << endl;
    return 0;
}