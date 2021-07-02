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
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    ll q,h,s,d,n;
    cin >> q >> h >> s >> d >> n;

    q*=4;
    h*=2;

    ll ans=0;
    if(min(2*q,min(2*h,min(2*s,d)))==d){
        ans+=n/2*d;
        if(n%2!=0){
            ans+=min(q,min(h,s));
        }
    }else{
        ans+=n*min(q,min(h,s));
    }

    cout << ans << endl;

    return 0;
}
