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

ll comb(ll l){
    ll ans=1;
    ll nowl=l;
    ll nowr=2;
    while(nowl>=l-10){
        ans*=nowl;
        nowl--;
        while(nowr<12 && ans%nowr==0){
            ans/=nowr;
            nowr++;
        }
    }
    return ans;
}

ll combination(ll n, ll r){
    vector<vector<ll>> v(n+1, vector<ll>(n+1,0));
    for(int i=0;i<v.size();i++){
        v[i][0]=1;
        v[i][i]=1;
    }
    for(int i=1;i<v.size();i++){
        for(int j=1;j<r+1;j++){
            v[i][j]=(v[i-1][j-1]+v[i-1][j]);
        }
    }
    return v[n][r];
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll l;
    cin >> l;
    // cout << comb(l-1) << endl;
    cout << combination(l-1, 11) << endl;
    return 0;
}
