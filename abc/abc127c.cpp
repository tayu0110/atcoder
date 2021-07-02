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
    int n,m;
    cin >> n >> m;

    vector<pii> lr(m);
    for(auto &x:lr){
        cin >> x.first >> x.second;
    }

    int minval=0,maxval=inf;
    for(int i=0;i<m;i++){
        minval=max(minval, lr[i].first);
        maxval=min(maxval, lr[i].second);
    }

    int ans = maxval-minval;
    if(ans<0)cout << 0 << endl;
    else cout << ans+1 << endl;

    return 0;
}
