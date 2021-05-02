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
    int n,m;
    ll k;
    cin >> n >> m >> k;
    vector<ll> a(n),b(m);
    ll nowt=0;
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    for(int i=0;i<m;i++){
        cin >> b[i];
        nowt+=b[i];
    }
    int j=m;
    int ans=0;
    for(int i=0;i<n+1;i++){
        while(j>0 && nowt>k){
            j--;
            nowt-=b[j];
        }
        if(nowt>k)break;
        ans=max(ans, i+j);
        if(i==n)break;
        nowt+=a[i];
    }
    cout << ans << endl;
}
