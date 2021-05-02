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

ll n,x;
vector<ll> len(51),p(51);

ll f(ll i,ll j){
    if(i==0) return (j<=0 ? 0 : 1);
    else if(j<=1+len[i-1]) return f(i-1,j-1);
    else return p[i-1]+1+f(i-1, j-2-len[i-1]);
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    cin >> n >> x;
    len[0]=1;
    p[0]=1;
    for(int i=1;i<=n;i++){
        len[i]=len[i-1]*2+3;
        p[i]=p[i-1]*2+1;
    }
    cout << f(n,x) << endl;
    // int now=1;
    // ll ans=0;
    // ll l=1,r=len[n];
    // if(x==l){
    //     cout << 0 << endl;
    //     return 0;
    // }else if(x==len[n]){
    //     cout << p[n] << endl;
    //     return 0;
    // }else{
    //     bool flag=false;
    //     while(now<=n){
    //         if((l+r)/2==x){
    //             ans+=p[n-now]+1;
    //             cout << ans << endl;
    //             return 0;
    //         }else if((l+r)/2>x){
    //             r=(l+r)/2;
    //             flag=false;
    //         }else{
    //             l=(l+r)/2;
    //             ans+=p[n-now]+1;
    //             flag=true;
    //         }
    //         now++;
    //         // cout << ans << endl;
    //     }
    //     // cout << "l: " << l << " r: " << r << endl;
    //     if(flag)ans++;
    //     else if(!flag && r-x<2)ans++;
    //     cout << ans << endl;
    // }
    return 0;
}
