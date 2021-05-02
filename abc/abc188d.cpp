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
    ll c;
    cin >> n >> c;
    vector<pair<pll,ll>> abc(n);
    map<ll,ll> mp;
    for(int i=0;i<n;i++){
        ll a,b,c;
        cin >> a >> b >> c;
        abc[i].first.first=a;
        abc[i].first.second=b;
        abc[i].second=c;
        if(mp.find(a)==mp.end()) mp.insert(make_pair(a,c));
        else mp[a]+=c;
        if(mp.find(b+1)==mp.end()) mp.insert(make_pair(b+1,-c));
        else mp[b+1]-=c;
    }
    map<ll,ll> cst;
    ll nowc=mp.begin()->second;
    ll prev=mp.begin()->first;
    ll now=0;
    ll ans=0;
    auto it=mp.begin();
    it++;
    for(it;it!=mp.end();it++){
        now=it->first;
        // cout << "now:" << now << " nowc: " << nowc << endl;
        // if(cst.find(now)==cst.end()) cst.insert(make_pair(now, nowc));
        // else cst[now]+=nowc;
        if(c<nowc) ans+=c*(now-prev);
        else ans+=nowc*(now-prev);
        prev=now;
        nowc+=it->second;
    }
    cout << ans << endl;
    return 0;
}
