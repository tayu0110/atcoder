#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<62)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int n,k;
    cin >> n >> k;

    vector<ll> a(n);
    for(auto &x:a)
        cin >> x;
    
    sort(a.begin(),a.end());

    ll ans=1;
    int minus=0;
    for(int i=n-1;i>n-1-k;i--){
        if(a[i]<0){
            a[i]*=-1;
            minus++;
        }
        ans=(ans*a[i])%MOD;
    }

    if(minus%2==1){
        ans=(ans*-1)%MOD;
    }

    cout << ans << endl;

    return 0;
}
