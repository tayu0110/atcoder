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

struct UnionFind{
    vector<ll> par;

    UnionFind(int n){
        par = vector<ll>(n, -1);
    }
    int root(int x){
        if(par[x] < 0) return x;
        return par[x] = root(par[x]);
    }
    bool merge(int x, int y){
        int rx = root(x);
        int ry = root(y);
        if(rx == ry) return false;
        if(par[rx] > par[ry]) swap(x, y);
        par[rx] += par[ry];
        par[ry] = rx;
        return true;
    }
    bool isSame(int x, int y){
        int rx = root(x);
        int ry = root(y);
        return rx == ry;
    }
    ll size(int x){
        return -par[root(x)];
    }
};

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll n,m;
    cin >> n >> m;
    vector<ll> a(m), b(m);
    for(int i=0;i<m;i++){
        cin >> a[i] >> b[i];
    }
    UnionFind uf(n+1);
    vector<ll> ans(0);
    ans.push_back((n*(n-1))/2);
    for(int i=m-1;i>=0;i--){
        int x=a[i],y=b[i];
        ll psz=ans.back();
        if(uf.root(x)!=uf.root(y))ans.push_back(psz-(uf.size(x)*uf.size(y)));
        else ans.push_back(psz);
        uf.merge(x,y);
    }
    for(int i=m-1;i>=0;i--){
        cout << ans[i] << endl;
    }
    return 0;
}
