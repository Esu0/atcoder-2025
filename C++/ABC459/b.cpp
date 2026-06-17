#include<iostream>
using namespace std;

char to_num(char c) {
    char r;
    if (c <= 'c') r = '2';
    else if (c <= 'f') r = '3';
    else if (c <= 'i') r = '4';
    else if (c <= 'l') r = '5';
    else if (c <= 'o') r = '6';
    else if (c <= 's') r = '7';
    else if (c <= 'v') r = '8';
    else r = '9';
    return r;
}
int main(void) {
    int N;
    cin>>N;
    for (int i=0;i<N;i++) {
        string S;
        cin>>S;
        cout << to_num(S[0]);
    }
    cout << endl;
    return 0;
}