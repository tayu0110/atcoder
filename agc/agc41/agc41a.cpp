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
    ll n,a,b;
    cin >> n >> a >> b;

    ll ans=0;
    if((b-a-1)%2==0){
        ans=min(a-1,n-b)+1+(b-a-1)/2;
    }else{
        ans=(b-a-1)/2+1;
    }

    cout << ans << endl;

    return 0;
}
