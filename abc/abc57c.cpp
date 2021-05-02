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

ll digits(ll n){
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
    ll n;
    cin >> n;
    ll now=1;
    ll ans=INF;
    while(now*now<=n){
        if(n%now!=0){
            now++;
            continue;
        }
        ll b=n/now;
        ans=min(ans, max(digits(b),digits(now)));
        now++;
    }
    cout << ans << endl;
    return 0;
}
