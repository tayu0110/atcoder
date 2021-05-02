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
#define INF (1LL<<60)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    ll x,k,d;
    cin >> x >> k >> d;

    ll ans=0;
    if((abs(x)/d)>=k){
        cout << abs(x)-d*k << endl;
        return 0;
    }else{
        ll nk = k-abs(x)/d;
        ll nx;
        if(x>0){
            nx = x-d*(x/d);
        }else{
            nx = x+d*(abs(x)/d);
        }
        if(nk%2==0){
            ans=abs(nx);
        }else{
            ans=abs(abs(nx)-d);
        }
    }

    cout << ans << endl;

    return 0;
}
