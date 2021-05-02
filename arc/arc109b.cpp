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

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll n;
    cin >> n;

    ll r=2*sqrtl(n+1);
    ll l=r/2;
    while(r-l>1){
        if(l*(l+1)/2==n+1)break;
        if(l*(l+1)/2<=n+1){
            l+=(r-l)/2;
        }else{
            ll temp=r;
            r=l+1;
            l-=(temp-l)/2;
        }
    }
    ll k;
    if(r*(r+1)/2>n+1 && l*(l+1)/2>n+1){
        k=l-1;
    }else if(r*(r+1)/2<=n+1){
        k=r;
    }else{
        k=l;
    }

    while(k*(k+1)/2<=n+1){
        if((k+1)*(k+2)/2<=n+1)k++;
        else break;
    }
    cout << n-k+1 << endl;
    return 0;
}
