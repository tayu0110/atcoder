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
    int n,m;
    cin >> n >> m;
    Graph t(n+1);
    for(int i=0;i<m;i++){
        int a, b;
        cin >> a >> b;
        t[a].push_back(b);
        t[b].push_back(a);
    }
    int k;
    cin >> k;
    vector<int> c(k);
    set<int> cc;
    for(int i =0; i <k; i++){
        cin >> c[i];
        cc.insert(c[i]);
    }
    ll ans=INF;
    for(int i=0;i<k;i++){
        int start=c[i];
        queue<int> nt;
        nt.push(start);
        set<int> ck;
        ck.insert(start);
        set<int> cl;
        vector<int> d(n+1);
        ll sum=0;
        while(!nt.empty()){
            int now=nt.front();
            nt.pop();
            for(int i=0;i<t[now].size();i++){
                int j=t[now][i];
                if(ck.find(j)!=ck.end()) continue;
                ck.insert(j);
                nt.push(j);
                d[j]=d[now]+1;
                if(cc.find(j)!=cc.end()){
                    sum+=d[j];
                    cl.insert(j);
                }
            }
        }
        if(cl.size()==k-1) ans=min(ans, sum);
    }
    if(ans!=INF) cout << ans+1 << endl;
    else cout << -1 << endl;
    return 0;
}
