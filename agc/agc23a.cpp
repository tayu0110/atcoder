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
    vector<ll> sum(n+1);
    map<ll, ll> ck;
    ck.insert(make_pair(0,1));
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(i==0)sum[i+1]=a[i];
        else sum[i+1]=a[i]+sum[i];
        if(ck.find(sum[i+1])==ck.end()) ck.insert(make_pair(sum[i+1], 1));
        else ck[sum[i+1]]++;
    }
    ll ans=0;
    for(auto it=ck.begin();it!=ck.end();it++){
        ll num=it->second;
        ans+=num*(num-1)/2;
    }
    cout << ans << endl;
    return 0;
}
