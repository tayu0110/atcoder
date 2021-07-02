//比の利用の場合、倍数約数に特に気を配る。
#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    vector<pii> ta(n);
    for(auto &x:ta)
        cin >> x.first >> x.second;
    
    ll t=1,a=1;
    for(auto x:ta){
        ll gcd=max(t%x.first==0?t/x.first:t/x.first+1,a%x.second==0?a/x.second:a/x.second+1);
        t=gcd*x.first;
        a=gcd*x.second;
    }

    cout << t+a << endl;

    return 0;
}
