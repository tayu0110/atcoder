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

struct G {
    struct E {
        int rev, from, to, cap;
        E(int r, int f, int t, int c) : rev(r), from(f), to(t), cap(c) {}
    };
    vector<vector<E>> list;
    G(int n = 0) : list(n) {}
    size_t size() {
        return list.size();
    }
    vector<E> &operator [] (int i) {
        return list[i];
    }
    E& redge(const E &e) {
        return list[e.to][e.rev];
    }
    void run_flow(E &e, int f) {
        e.cap--;
        redge(e).cap++;
    }
    void addedge(int from, int to, int cap) {
        int fromrev = (int)list[from].size();
        int torev = (int)list[to].size();
        list[from].push_back(E(torev, from, to, cap));
        list[to].push_back(E(fromrev, to, from, 0));
    }
};

struct FF {
    vector<bool> ck;
    int dfs(G &g, int v, int t, int f) {
        if(v == t) return f;
        ck[v] = true;
        for(auto &e : g[v]) {
            if(ck[e.to]) continue;
            if(e.cap == 0) continue;
            int flow = dfs(g, e.to, t, min(f, e.cap));
            if(flow == 0) continue;
            g.run_flow(e, flow);
            return flow;
        }
        return 0;
    }
    int solve(G &g, int s, int t) {
        int res = 0;
        ck.assign((int)g.size(), false);
        while(dfs(g, s, t, inf)) {
            ck.assign((int)g.size(), false);
            res++;
        }
        return res;
    }
};

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n,g,e;
    cin >> n >> g >> e;
    set<int> p;
    for(int i = 0; i < g; i++) {
        int k;
        cin >> k;
        p.insert(k);
    }
    G t(n+1);
    for(int i = 0; i < e; i++) {
        int a, b;
        cin >> a >> b;
        if(a > b) swap(a, b);
        t.addedge(a, b, 1);
    }
    for(auto h : p) {
        t.addedge(h, n, 1);
    }
    // cout << "reached" << endl;
    FF ff;
    cout << ff.solve(t, 0, n) << endl;
    return 0;
}
