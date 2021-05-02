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
    ll sum=0;
    for(int i=0;i<n;i++){
        cin >> a[i];
        a[i]-=i+1;
    }
    sort(a.begin(),a.end());
    if(n%2==0){
        ll ans1=0,ans2=0;
        ll b1=a[n/2];
        ll b2=a[n/2-1];
        for(int i=0;i<n;i++){
            ans1+=abs(a[i]-b1);
            ans2+=abs(a[i]-b2);
        }
        cout << min(ans1, ans2) << endl;
    }else {
        ll ans=0;
        ll b=a[n/2];
        for(int i=0;i<n;i++){
            ans+=abs(a[i]-b);
        }
        cout << ans << endl;
    }
    return 0;
}
