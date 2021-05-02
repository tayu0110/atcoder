#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
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
    vector<ll> w(n);
    ll maxw=0;
    for(auto &x:w){
        cin >> x;
        maxw=max(maxw,x);
    }
    vector<pll> vl(m);
    ll minv=INF;
    for(auto &x:vl){
        ll l,v;
        cin >> l >> v;
        x.first=v;x.second=l;
        minv=min(minv,x.first);
    }
    if(maxw>minv){cout << -1 << endl;return 0;}

    sort(w.begin(), w.end());
    sort(vl.begin(), vl.end());

    vector<ll> dist(n,0);
    for(int i=0;i<m;i++){
        ll sumw=w[0];
        for(int j=1;j<n;j++){
            if(sumw+w[j]<=vl[i].first){
                sumw+=w[j];
                dist[j]=max((0LL),min(dist[j],vl[i].second));
                vl[i].second-=dist[j];
            }else{
                sumw=w[j];
                dist[j]=max(dist[j],vl[i].second);
            }
        }
    }

    ll ans=0;
    for(int i=0;i<n;i++){
        ans+=dist[i];
    }
    cout << ans << endl;

    return 0;
}
