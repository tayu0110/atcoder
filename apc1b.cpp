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
    ll asum=0,bsum=0;
    for(int i=0;i<n;i++){
        cin >> a[i];
        asum+=a[i];
    }
    for(int i=0;i<n;i++){
        cin >> b[i];
        bsum+=b[i];
    }
    ll x=0,y=0;
    for(int i=0;i<n;i++){
        if(a[i]>b[i]) x+=a[i]-b[i];
        else y+=(b[i]-a[i]+1)/2;
    }
    if((bsum-asum)>=y && (bsum-asum)>=x) cout << "Yes" << endl;
    else cout << "No" << endl;
    return 0;
}
