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
    int x,y;
    cin >> x >> y;
    if((x+y)%3!=0){
        cout << 0 << endl;
        return 0;
    }
    if(2*x-y<0 || 2*y-x<0){
        cout << 0 << endl;
        return 0;
    }
    ll n=(2*y-x)/3, m=(2*x-y)/3;
    ll a=1,b=1;
    for(ll i=0;i<n;i++){
        a*=(n+m)-i;
        a%=MOD;
        b*=n-i;
        b%=MOD;
    }
    b=modPow(b, MOD-2, MOD);
    cout << a*b%MOD << endl;
    return 0;
}
