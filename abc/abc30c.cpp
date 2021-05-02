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
    ll n,m,x,y;
    cin >> n >> m >> x >> y;
    vector<ll> a(n), b(m);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    for(int i=0;i<m;i++){
        cin >> b[i];
    }
    vector<pll> f(0);
    int ai=0,bi=0;
    while(ai<n || bi<m){
        if((ai < n) && (bi >= m || a[ai] < b[bi])) {
            f.push_back(make_pair(a[ai], 0));
            ai++;
        } else {
            f.push_back(make_pair(b[bi], 1));
            bi++;
        }
    }
    f.push_back({INF, 0});
    f.push_back({INF, 1});
    ll now=0, tm=a[0];
    ll xy[2] = {x, y};
    ll ans = 0;
    for(int i=0;i<f.size();i++) {
        if(f[i].second == now) continue;
        if(f[i].first < tm+xy[now]) continue;
        tm  = f[i].first;
        now ^= 1;
        // cout << "now: " << now << " tm: " << tm << endl;
        ans++;
    }
    cout << ans/2 << endl;
    return 0;
}
