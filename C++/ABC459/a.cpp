#include<iostream>
using namespace std;

int main(void) {
    int X;
    cin >> X;
    char s[] = "HelloWorld";
    for (int i = 0; i < 10; i++) if(X != i + 1) cout << s[i];
    cout << endl;
    return 0;
}