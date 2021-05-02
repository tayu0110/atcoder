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

struct Edge {
    int to;
    long long weight;
    Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

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
    vector<ll> t(n);
    map<ll, ll> mp;
    for(int i=0;i<n;i++){
        cin >> t[i];
        mp[t[i]]++;
    }
    sort(t.begin(), t.end());
    ll tm=0,ans=0;
    for(int i=0;i<n;i++){
        tm+=t[i];
        ans+=tm;
    }
    ll w=1;
    for(auto it=mp.begin();it!=mp.end();it++){
        ll x=1;
        for(ll i=it->second;i>=2;i--){
            x*=i;
            x%=MOD;
        }
        w*=x;
        w%=MOD;
    }
    cout << ans << endl;
    cout << w << endl;
    return 0;
}
