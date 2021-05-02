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
    ll k;
    cin >> n >> k;
    vector<int> a(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    vector<int> f(2*n);
    for(int i=0;i<2*n;i++){
        for(int j=0;j<i;j++){
            if(a[i%n]<a[j%n])f[i]++;
        }
    }
    ll d=0;
    for(int i=1;i<n;i++){
        d+=f[i];
    }
    ll ans=0;
    for(int i=n;i<2*n;i++){
        ans+=f[i];
    }
    ans-=d;
    cout << ((d*k%MOD) + (k*(k-1)/2)%MOD*ans%MOD)%MOD << endl;
    return 0;
}
