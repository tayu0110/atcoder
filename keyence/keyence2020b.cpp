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
    int n;
    cin >> n;
    vector<pll> r(n);
    for(int i=0;i<n;i++){
        ll x,l;
        cin >> x >> l;
        ll s=x-l,t=x+l;
        r[i].first=t,r[i].second=s;
    }
    sort(r.begin(),r.end());
    ll s,t=r[0].first;
    ll ans=n;
    for(int i=1;i<n;i++){
        s=r[i].second;
        if(s<t) ans--;
        else t=r[i].first;
    }
    cout << ans << endl;
    return 0;
}
