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

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n,m;
    cin >> n >> m;
    vector<pii> e(m);
    vector<vector<int>> g(n+1, vector<int>(0));
    for(int i=0;i<m;i++){
        int f,s;
        cin >> f >> s;
        e[i].first=f;
        e[i].second=s;
        g[f].push_back(s);
        g[s].push_back(f);
    }
    int ans=0;
    for(int i=0;i<m;i++){
        int f=e[i].first,s=e[i].second;
        queue<int> nt;
        nt.push(f);
        set<int> ck;
        bool flag=false;
        while(!nt.empty()){
            int nd=nt.front();
            nt.pop();
            if(nd==s){
                flag=true;
                break;
            }
            for(int j=0;j<g[nd].size();j++){
                if(nd==f && g[nd][j]==s)continue;
                if(ck.find(g[nd][j])!=ck.end())continue;
                nt.push(g[nd][j]);
                ck.insert(g[nd][j]);
            }
        }
        if(!flag)ans++;
    }
    cout << ans << endl;
    return 0;
}
