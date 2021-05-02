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
    ll ans=0;
    set<ll> ck;
    for(ll i=1;i*i<=n;i++){
        if(n%i!=0)continue;
        ll lm=n/i-1,sm=i-1;
        // cout << "lm: " << lm << " sm: " << sm << endl;
        if(ck.find(lm)==ck.end() && lm!=0 && n%lm==n/lm)ans+=lm;
        ck.insert(lm);
        if(ck.find(sm)==ck.end() && sm!=0 && n%sm==n/sm)ans+=sm;
        ck.insert(sm);
    }
    cout << ans << endl;
    return 0;
}
