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
    ll a,b;
    int k;
    cin >> a >> b >> k;

    for(ll i=a;i<min(a+k,b);i++){
        cout << i << endl;
    }

    for(int i=b-k+1;i<=b;i++){
        if(i<min(a+k,b))continue;
        cout << i << endl;
    }

    return 0;
}
