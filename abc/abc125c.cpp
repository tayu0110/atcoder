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
    vector<ll> a(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    vector<ll> l(n+1),r(n+1);
    l[0]=0;
    r[n]=0;
    for(int i=0;i<n;i++){
        if(l[i]==0) l[i+1]=a[i];
        else l[i+1]=gcd(l[i],a[i]);
    }
    for(int i=n-1;i>=0;i--){
        if(r[i+1]==0) r[i]=a[i];
        else r[i]=gcd(r[i+1],a[i]);
    }
    vector<ll> m(n);
    ll ans=0;
    for(int i=0;i<n;i++){
        if(l[i]==0) m[i]=r[i+1];
        else if(r[i+1]==0) m[i]=l[i];
        else m[i]=gcd(l[i],r[i+1]);
        ans = max(ans,m[i]);
    }
    // for(int i=0;i<n+1;i++) cout << l[i] << " ";
    // cout << endl;
    // for(int i=0;i<n+1;i++) cout << r[i] << " ";
    // cout << endl;
    cout << ans << endl;
    return 0;
}
