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
    ll n,m;
    cin >> n >> m;
    if(abs(n-m)>1){
        cout << 0 << endl;
        return 0;
    }
    if(n==m){
        ll ans=1;
        for(ll i=0;i<n;i++){
            ans*=n-i;
            ans%=MOD;
        }
        cout << (((ans*ans)%MOD)*2)%MOD << endl;        
    }else{
        if(m>n)swap(m,n);
        ll ans=1,mans=1;
        for(ll i=0;i<n;i++){
            ans*=n-i;
            ans%=MOD;
            if(i==0)continue;
            mans*=n-i;
            mans%=MOD;
        }
        cout << (ans*mans)%MOD << endl;
    }
    return 0;
}
