#include<iostream>
#include<vector>
#include<algorithm>
using namespace std;

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

using Edge = pair<int, pair<int, int>>;

int main() {
    int N, M;
    cin >> N >> M;
    vector<Edge> edges(M);
    for(int i = 0; i < M; i++) {
        int u, v, w;
        cin >> u >> v >> w;
        edges[i] = Edge(w, make_pair(u, v));
    }

    sort(edges.begin(), edges.end());

    long long res = 0;
    UnionFind uf(N);
    for(int i = 0; i < M; i++) {
        int w = edges[i].first;
        int u = edges[i].second.first;
        int v = edges[i].second.second;

        if(uf.isSame(u, v)) continue;

        res += w;
        uf.merge(u, v);
    }
    cout << res << endl;

    return 0;
}