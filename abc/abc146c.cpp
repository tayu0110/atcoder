#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int digits(ll n){
    int exp=0;
    while(n>0){
        n/=10;
        exp++;
    }
    return exp;
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll a,b,x;
    cin >> a >> b >> x;
    ll ans=0;

    ll l=1000000000,s=0;
    int d=digits(l);
    if(x-b*d>=a*l){
        cout << l << endl;
        return 0;
    }
    l/=2;
    while(l-s>0){
        int d=digits(l);
        if(x-b*d>=a*l){
            ll temp=s;
            s=l;
            l+=(ll)ceill((ld)(l-temp)/2);
        }else{
            l-=(ll)ceill((ld)(l-s)/2);
        }
    }
    cout << l << endl;

    // for(int d=1;d<10;d++){
    //     ll nx=x-b*d;
    //     if(nx<0)continue;
    //     ll n=nx/a;
    //     int exp=digits(n);
    //     if(exp==d)ans=max(ans,n);
    //     if(d==9 && d<exp)ans=1000000000;
    //     // cout << "d: " << d << " exp: " << exp << endl;
    // }
    return 0;
}
