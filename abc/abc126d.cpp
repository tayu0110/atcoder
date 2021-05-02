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

set<int> ck;
ll d[100005];
map<pii, ll> mp;
void dfs(int node, vector<vector<int>> &t){
    if(t[node].size()==0)return;
    // if(ck.find(node)!=ck.end())return;
    for(int i=0;i<t[node].size();i++){
        int j=t[node][i];
        if(ck.find(j)!=ck.end())continue;
        ck.insert(j);
        d[j]=d[node]+mp[make_pair(node, j)];
        dfs(j, t);
    }
    return;
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n;
    cin >> n;
    vector<vector<int>> t(n,vector<int>(0));
    for(int i=0;i<n-1;i++){
        int u,v;
        ll w;
        cin >> u >> v >> w;
        u--;v--;
        mp.insert(make_pair(make_pair(u,v),w));
        mp.insert(make_pair(make_pair(v,u),w));
        t[u].push_back(v);
        t[v].push_back(u);
    }
    dfs(0,t);
    // cout << "reached" << endl;
    for(int i=0;i<n;i++){
        // cout << "i: " << i << " d[i]: " << d[i] << endl;
        if(d[i]%2==0) cout << 0 << endl;
        else cout << 1 << endl;
    }
    return 0;
}
