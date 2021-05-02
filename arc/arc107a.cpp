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

    ll a,b,c;
    cin >> a >> b >> c;

    ll mod=998244353;
    ll asum=(a*(a+1)/2)%mod;
    ll bsum=(b*(b+1)/2)%mod;
    ll csum=(c*(c+1)/2)%mod;
    
    ll ans=0;
    ans=(asum*bsum)%mod*csum%mod;
    cout << ans << endl;

    return 0;
}
