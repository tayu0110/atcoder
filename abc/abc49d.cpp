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
    vector<int> par;

    UnionFind(int n){
        par = vector<int>(n, -1);
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
    int size(int x){
        return -par[root(x)];
    }
};

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n,k,l;
    cin >> n >> k >> l;
    UnionFind r(n+1), t(n+1);
    for(int i=0;i<k;i++){
        int p,q;
        cin >> p >> q;
        r.merge(p,q);
    }
    for(int i=0;i<l;i++){
        int r,s;
        cin >> r >> s;
        t.merge(r,s);
    }
    map<pii,int> mp;
    for(int i=1;i<n+1;i++){
        mp[make_pair(r.root(i), t.root(i))]++;
    }
    for(int i=1;i<n+1;i++){
        cout << mp[make_pair(r.root(i), t.root(i))] << " ";
    }
    cout << endl;
    return 0;
}
