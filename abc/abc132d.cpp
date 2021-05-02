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

ll v[2005][2005];
bool check[2005][2005];

void combination(ll n, ll r){
    // vector<vector<ll>> v(n+1, vector<ll>(n+1,0));
    if(check[n][r]) return;
    for(ll i=0;i<n+1;i++){
        v[i][0]=1;
        v[i][i]=1;
        check[i][0]=true;
        check[i][i]=true;
    }
    for(ll i=1;i<n+1;i++){
        for(ll j=1;j<r+1;j++){
            if(check[i][j])continue;
            v[i][j]=(v[i-1][j-1]+v[i-1][j])%MOD;
            check[i][j]=true;
        }
    }
    return;
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll n,k;
    cin >> n >> k;
    for(int i=0;i<=n;i++)for(int j=0;j<=n;j++)check[i][j]=false;
    combination(n+1,n+1);
    // for(int i=0;i<=n;i++){
    //     for(int j=0;j<=n;j++){
    //         cout << v[i][j] << " ";
    //     }
    //     cout << endl;
    // }
    n-=k;
    ll ans=1;
    for(ll i=1;i<=k;i++){
        ans=1;
        ll c=v[n+1][i];
        ll ck=v[k-1][i-1];
        ans=c*ck%MOD;
        cout << ans << endl;
    }
    return 0;
}
