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
    ll a,b,c;
    cin >> a >> b >> c;

    ll ans=0;
    if(a%2==0 || b%2==0 || c%2==0){
        cout << 0 << endl;
        return 0;
    }else{
        ans=min(a*b,min(a*c,c*b));
    }

    cout << ans << endl;

    return 0;
}
