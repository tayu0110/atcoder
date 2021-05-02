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
    ll m;
    cin >> n >> m;
    vector<ll> a(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    vector<ll> t(0);
    for(int i=0;i<n;i++){
        for(int j=0;j<n;j++){
            t.push_back(a[i]+a[j]);
        }
    }
    for(int i=0;i<n;i++) t.push_back(a[i]);
    t.push_back(0);
    sort(t.begin(),t.end());
    ll ans=0;
    for(int i=0;i<t.size();i++){
        ll sum=m-t[i];
        if(sum<0) continue;
        int l=0,r=t.size()-1;
        while(r-l>1){
            int mid=(l+r)/2;
            if(t[mid]==sum){
                cout << m << endl;
                return 0;
            }else if(t[mid]>sum){
                r=mid;
            } else{
                l=mid;
            } 
        }
        if(t[i]+t[l]<=m) ans=max(ans, t[i]+t[l]);
        // cout << ans << endl;
    }
    cout << ans << endl;
    return 0;
}
