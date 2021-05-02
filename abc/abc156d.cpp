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

ll modPow(long long a, long long n, long long p) {
    if (n == 0) return 1;
    if (n == 1) return a % p;
    if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
    long long t = modPow(a, n / 2, p);
    return (t * t) % p;
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll n,a,b;
    cin >> n >> a >> b;
    ll ans=modPow(2, n, MOD);
    if(ans==0)ans=MOD-1;
    else ans--;
    ll xa=1,ya=1,xb=1,yb=1;
    for(ll i=0;i<a;i++){
        xa*=n-i;
        xa%=MOD;
        ya*=a-i;
        ya%=MOD;
    }
    for(ll i=0;i<b;i++){
        xb*=n-i;
        xb%=MOD;
        yb*=b-i;
        yb%=MOD;
    }
    ll ca=xa*modPow(ya, MOD-2, MOD), cb=xb*modPow(yb, MOD-2, MOD);
    ans-=ca+cb;
    if(ans<0)while(ans<0)ans+=MOD;
    cout << ans << endl;
    return 0;
}
