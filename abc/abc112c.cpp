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

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n;
    cin >> n;
    vector<pair<ll,pll>> hxy(n);
    for(int i=0;i<n;i++){
        ll x,y,h;
        cin >> x >> y >> h;
        hxy[i]=make_pair(h, make_pair(x,y));
    }
    sort(hxy.begin(), hxy.end(), greater<pair<ll,pll>>());
    pair<ll,pll> ans;
    for(ll cx=0;cx<=100;cx++){
        bool oflag=false;
        for(ll cy=0;cy<=100;cy++){
            bool iflag=false;
            ll height=-1;
            for(ll i=0;i<n;i++){
                ll h=hxy[i].first;
                ll x=hxy[i].second.first;
                ll y=hxy[i].second.second;
                if(h!=0){
                    if(height==-1) height=h+abs(x-cx)+abs(y-cy);
                    else if(height!=h+abs(x-cx)+abs(y-cy)) break;
                }else{
                    if(height==-1) height=1;
                    else if(height-abs(x-cx)-abs(y-cy)>1) break;
                }
                if(i==n-1)iflag=true;
            }
            if(iflag){
                oflag=true;
                ans=make_pair(cx,make_pair(cy,height));
                break;
            }
        }
        if(oflag)break;
    }
    cout << ans.first << " " << ans.second.first << " " << ans.second.second << endl;
    return 0;
}
