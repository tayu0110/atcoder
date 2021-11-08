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
    vector<ll> a(n),b(n);
    vector<ll> diff(n);
    ll suma=0,sumb=0;
    for(int i=0;i<n;i++){
        cin >> a[i];
        suma+=a[i];
    }
    for(int i=0;i<n;i++){
        cin >> b[i];
        sumb+=b[i];
        diff[i]=a[i]-b[i];
    }
    if(suma<sumb){
        cout << -1 << endl;
        return 0;
    }
    sort(diff.begin(),diff.end());
    int ans=0;
    ll minu=0;
    for(int i=0;diff[i]<0;i++){
        minu+=abs(diff[i]);
        ans++;
    }
    for(int i=n-1;minu>0;i--){
        minu-=diff[i];
        ans++;
    }
    cout << ans << endl;
    return 0;
}
