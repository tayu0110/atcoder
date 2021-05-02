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
    int n,m,x;
    cin >> n >> m >> x;
    vector<int> c(n);
    vector<vector<int>> a(n,vector<int>(m));
    for(int i=0;i<n;i++){
        cin >> c[i];
        for(int j=0;j<m;j++){
            cin >> a[i][j];
        }
    }
    ll ans = INF;
    for(ll bit = 0; bit < (1<<n); bit++){
        vector<int> sum(m);
        ll cost=0;
        for(int i=0;i<n;i++){
            if(bit & (1<<i)){
                cost+=c[i];
                for(int j=0;j<m;j++){
                    sum[j]+=a[i][j];
                }
            }
        }
        bool flag=true;
        for(int i=0;i<m;i++){
            if(sum[i]<x)flag=false;
        }
        if(flag)ans=min(ans,cost);
    }
    if(ans==INF)cout << -1 << endl;
    else cout << ans << endl;
    return 0;
}
