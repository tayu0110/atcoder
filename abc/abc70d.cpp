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
    vector<vector<int>> t(n, vector<int>(0));
    map<pii, ll> mp;
    for(int i=0;i<n-1;i++){
        int a,b;
        ll c;
        cin >> a >> b >> c;
        a--;b--;
        t[a].push_back(b);
        t[b].push_back(a);
        mp.insert(make_pair(make_pair(a,b), c));
        mp.insert(make_pair(make_pair(b,a), c));
    }
    int q,k;
    cin >> q >> k;
    k--;
    queue<int> nt;
    nt.push(k);
    set<int> ck;
    ck.insert(k);
    vector<ll> d(n+1);
    d[k]=0;
    while(!nt.empty()){
        int now=nt.front();
        nt.pop();
        for(int i=0;i<t[now].size();i++){
            int j=t[now][i];
            if(ck.find(j)!=ck.end())continue;
            ck.insert(j);
            nt.push(j);
            ll dist=mp[make_pair(now,j)];
            d[j]=d[now]+dist;
        }
    }
    for(int i=0;i<q;i++){
        int x,y;
        cin >> x >> y;
        x--;y--;
        cout << d[x] + d[y] << endl;
    }
    return 0;
}
