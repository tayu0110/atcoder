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
    int n,q;
    cin >> n >> q;
    vector<vector<ll>> child(n+1, vector<ll>(0));
    for(int i=0;i<n-1;i++){
        int a,b;
        cin >> a >> b;
        child[a].push_back(b);
        child[b].push_back(a);
    }
    vector<ll> ans(n+1,0);
    for(int i=0;i<q;i++){
        int p,x;
        cin >> p >> x;
        ans[p]+=x;
    }
    set<int> ck;
    queue<int> nt;
    nt.push(1);
    while(!nt.empty()){
        int now=nt.front();
        nt.pop();
        ck.insert(now);
        for(int i=0;i<child[now].size();i++){
            int c=child[now][i];
            if(ck.find(c)!=ck.end())continue;
            ans[c]+=ans[now];
            nt.push(c);
        }
    }
    for(int i=1;i<n+1;i++){
        cout << ans[i] << " ";
    }
    return 0;
}
