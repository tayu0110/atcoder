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
    vector<ll> a(n),sum(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(i==0)sum[i]=a[i];
        else sum[i]=sum[i-1]+a[i];
    }

    ll ans=INF;
    for(int i=0;i<n-1;i++){
        ll sn,ar;
        sn=sum[i],ar=sum[n-1]-sum[i];
        ans=min(ans,abs(sn-ar));
    }

    cout << ans << endl;

    return 0;
}
