#include<iostream>
#include<vector>

using namespace std;
using Graph = vector<vector<int>>;

vector<int> depth;
vector<int> subtree_size;
void dfs(const Graph &G, int v, int p = -1, int d = 0) {
    depth[v] = d;
    for(auto c : G[v]) {
        if(c == p) continue;
        dfs(G, c, v, d+1);
    }

    subtree_size[v] = 1;
    for(auto c : G[v]) {
        if(c == p) continue;
        subtree_size[v] += subtree_size[c];
    }
}

int main() {
    int N;
    cin >> N;

    Graph G(N);
    for(int i = 0; i < N-1; i++) {
        int a, b;
        cin >> a >> b;
        G[a].push_back(b);
        G[b].push_back(a);
    }

    int root = 0;
    depth.assign(N, 0);
    subtree_size.assign(N, 0);
    dfs(G, root);

    for(int v = 0; v < N; v++) {
        cout << v << ": depth = " << depth[v] << ", subtree_size = " << subtree_size[v] << endl;
    }

    return 0;
}