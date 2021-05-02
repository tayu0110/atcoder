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
    int n,m;
    cin >> n >> m;
    weightedGraph g(n+1);
    for(int i=0;i<m;i++){
        ll a,b,c;
        cin >> a >> b >> c;
        g[a].push_back(Edge(b, -c));
    }
    // bool isINF = false;
    vector<ll> dist(n+1, INF);
    dist[1]=0;
    for(int i=0;i<n-1;i++){
        // bool update=false;
        for(int v=1;v<n+1;v++){
            if(dist[v]==INF) continue;
            for(auto e : g[v]) {
                if(dist[e.to] > dist[v]+e.weight){
                    dist[e.to] = dist[v]+e.weight;
                    // update=true;
                }
            }
        }
        // if(!update) break;
        // if(i==n-1 && update) isINF=true;
    }
    ll ans=dist[n];
    vector<bool> neg(n+1, false);
    for(int i=0;i<n;i++){
        for(int v=1;v<n+1;v++){
            for(auto e : g[v]){
                if(dist[v]==INF) continue;
                if(dist[e.to] > dist[v]+e.weight){
                    dist[e.to]=dist[v]+e.weight;
                    neg[e.to]=true;
                }
                if(neg[v]){
                    neg[e.to]=true;
                }
            }
        }
    }
    if(neg[n]) cout << "inf" << endl;
    else cout << -ans << endl;
    return 0;
}
