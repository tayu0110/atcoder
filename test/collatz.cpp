//collatz.cpp
#include<iostream>

using namespace std;
using ll = long long;

int main(){
    ll n;
    cin >> n;

    if(n<1 || n>(1LL<<60)){
        cout << "Input Error" << endl;
        return 0;
    }

    ll ans = n;
    ll count = 0;
    while(ans != 1){
        if(ans%2 == 1)  ans = ans*3+1;
        else            ans /= 2;
        cout << ans << endl;
        count++;
    }

    cout << "Count : " << count << endl;

    return 0;
}