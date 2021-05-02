#include<iostream>

using namespace std;

int main(){
    int n = (1<<29);

    int nn=n + n;

    cout << n << endl;
    cout << nn << endl;

    long long inf=(1LL<<60);
    long long infs=inf+inf;

    cout << inf << endl;
    cout << infs << endl;

    return 0;
}