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
    vector<ll> sum(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(i==0)sum[i]=a[i];
        else sum[i]=sum[i-1]+a[i];
    }
    vector<ll> ans(n);
    ll num=0;
    for(int i=1;i<n;i+=2){
        num+=a[i];
    }
    ans[0]=sum[n-1]-2*num;
    for(int i=1;i<n;i++){
        ans[i]=2*a[i-1]-ans[i-1];
    }
    for(int i=0;i<n;i++){
        cout << ans[i] << " ";
    }
    return 0;
}
