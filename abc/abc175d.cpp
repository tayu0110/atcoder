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
    // ll n, k;
    // cin >> n >> k;

    // vector<int> p(n);
    // for(auto &x:p)
    //     cin >> x;
    // vector<ll> c(n);
    // for(auto &x:c)
    //     cin >> x;
    
    // if(n>=k){
    //     ll maxval=-INF;
    //     for(int i=0;i<n;i++){
    //         ll nowval=0;
    //         int nowpt=i;
    //         for(int j=0;j<k;j++){
    //             nowval+=c.at(p.at(nowpt)-1);
    //             nowpt=p.at(nowpt)-1;
    //             maxval=max(maxval, nowval);
    //         }
    //     }
    //     cout << maxval << endl;
    //     return 0;
    // }else{
    //     ll maxval=-INF;
    //     vector<vector<ll>> dp(n, vector<ll>(k+1, 0));
    //     vector<bool> check(n,false);
    //     for(int i=0;i<n;i++){
    //         int nowpt=i;
    //         if(check.at(nowpt)==true)
    //             continue;
    //         else
    //             check.at(nowpt)=true;
    //         for(int j=1;j<=k;j++){
    //             dp.at(i).at(j)=dp.at(i).at(j-1)+c.at(p.at(nowpt)-1);
    //             nowpt=p.at(nowpt)-1;
    //             maxval=max(maxval, dp.at(i).at(j));
    //         }
    //     }
    //     cout << maxval << endl;
    //     return 0;
    // }

    // sunuke's answer
    int n, k;
    cin >> n >> k;
    vector<int> p(n), c(n);
    for(auto &x:p){
        cin >> x;
        x--;
    }
    for(auto &x:c)cin >> x;

    ll ans=-1e18;
    for(int si=0;si<n;si++){
        int x=si;
        vector<int> s;
        ll tot=0;
        while(1){
            x=p[x];
            s.push_back(c[x]);
            tot+=c[x];
            if(x==si)break;
        }
        int l=s.size();
        ll t=0;
        for(int i=0;i<l;i++){
            t+=s[i];
            if(i+1>k)break;
            ll now=t;
            if(tot>0){
                ll e=(k-(i+1))/l;
                now+=tot*e;
            }
            ans=max(ans, now);
        }
    }
    cout << ans << endl;

    return 0;
}
