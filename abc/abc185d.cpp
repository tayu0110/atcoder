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
    ll n;
    int m;
    cin >> n >> m;
    if(m==0){
        cout << 1 << endl;
        return 0;
    }
    vector<ll> a(m);
    vector<ll> diff(m+1);
    for(int i=0;i<m;i++){
        cin >> a[i];
    }
    sort(a.begin(),a.end());
    ll mdiff=INF;
    for(int i=1;i<m;i++){
        diff[i]=a[i]-a[i-1]-1;
        if(diff[i]!=0) mdiff=min(mdiff, diff[i]);
    }
    diff[0]=(a[0]-1 >=0 ? a[0]-1 : 0);
    if(diff[0]!=0) mdiff=min(mdiff,diff[0]);
    diff[m]=(n-a[m-1] >= 0 ? n-a[m-1] : 0);
    if(diff[m-1]>0) mdiff=min(mdiff,diff[m-1]);
    ll k=mdiff;
    // cout << k << endl;
    ll ans=0;
    if(k==INF) cout << 0 << endl;
    else{
        for(int i=0;i<m+1;i++){
            if(diff[i]%k==0)ans+=diff[i]/k;
            else ans+=diff[i]/k+1;
        }
        cout << ans << endl;
    }
    return 0;
}
