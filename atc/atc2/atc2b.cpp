//繰り返し二乗法の指数法則による実装と二進数による実装
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
using pll = pair<ll, ll>;

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

ll pow_mod(ll, ll, ll);
ll binary_pow_mod(ll, ll, ll);

int main(int argc,char* argv[]){
    ll n,m,p;
    cin >> n >> m >> p;

    cout << binary_pow_mod(n,p,m) << endl;

    return 0;
}

ll pow_mod(ll n,ll k,ll m){
    if(k==0)
        return 1;
    else if(k%2==1)
        return pow_mod(n,k-1,m)*n%m;
    else{
        ll t=pow_mod(n,k/2,m);
        return t*t%m;
    }
}

ll binary_pow_mod(ll n,ll k,ll m){
    ll r=1;                 //r=n^0
    for(;k>0;k>>=1){        //k>>=1はk=k>>1と同じ
        if(k&1)             //フラグが立っているとき、rにnをかける
            r=(r*n)%m;
        n=(n*n)%m;
    }

    return r;
}