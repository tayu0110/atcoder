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
    ll n;
    cin >> n;

    ll root=INF;
    for(ll i=1;i*i<=n;i++){
        if(n%i!=0){
            continue;
        }else{
            ll x=i-1,y=n/i-1;

            root=min(root,x+y);
        }
    }

    cout << root << endl;

    return 0;
}
