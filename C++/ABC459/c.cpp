#include<iostream>
using namespace std;

int cnt[3<<17], ans[6<<17];

int main(void) {
    int N,Q;
    cin>>N>>Q;
    int min = 0;
    ans[0] = N;
    while (Q--) {
        int t,x;
        cin>>t>>x;
        if (t == 1) {
            cnt[x]+=1;
            ans[cnt[x]]+=1;
            if (ans[min+1]==N)min+=1;
        } else {
            cout << ans[x + min] << "\n";
        }
    }
    cout << flush;
    return 0;
}