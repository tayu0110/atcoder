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
    int n;
    cin >> n;
    vector<vector<int>> tree(n, vector<int>(0));
    vector<pii> ab(n-1);
    for(int i=0;i<n-1;i++){
        int a,b;
        cin >> a >> b;
        a--;b--;
        ab[i]=make_pair(a,b);
        tree[a].push_back(b);
        tree[b].push_back(a);
    }
    queue<int> nt;
    set<int> ck;
    vector<set<int>> node(n);
    map<pii, int> mp;
    nt.push(0);
    ck.insert(0);
    int mx=1;
    while(!nt.empty()){
        int now=nt.front();
        nt.pop();
        int c=1;
        for(int i=0;i<tree[now].size();i++){
            int j=tree[now][i];
            if(ck.find(j)!=ck.end())continue;
            ck.insert(j);
            nt.push(j);
            while(node[now].find(c)!=node[now].end() || node[j].find(c)!=node[j].end())c++;
            mx=max(mx,c);
            node[now].insert(c);
            node[j].insert(c);
            mp.insert(make_pair(make_pair(now,j), c));
            mp.insert(make_pair(make_pair(j,now), c));
        }
    }
    cout << mx << endl;
    for(int i=0;i<n-1;i++){
        int a=ab[i].first,b=ab[i].second;
        cout << mp[make_pair(a,b)] << endl;
    }
    return 0;
}
