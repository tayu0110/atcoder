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
    int n,k;
    cin >> n >> k;

    vector<ll> h(n);
    for(auto &x:h)cin >> x;

    sort(h.begin(),h.end());
    
    int hmin=0,hmax=k-1;
    ll mindiff=INF;
    for(int i=k-1;i<h.size();i++){
        ll diff=h[hmax]-h[hmin];
        mindiff=min(mindiff, diff);
        hmin++;
        hmax++;
    }

    cout << mindiff << endl;

    return 0;
}
