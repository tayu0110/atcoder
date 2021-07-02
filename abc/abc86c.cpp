#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
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
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    cin >> n;
    vector<int> t(n);
    vector<pii> xy(n);
    for(int i=0;i<n;i++){
        cin >> t[i];
        cin >> xy[i].first >> xy[i].second;
    }

    bool ans=true;
    int prevx=0,prevy=0,prevtm=0;
    for(int i=0;i<n;i++){
        int xdist=abs(prevx-xy[i].first),ydist=abs(prevy-xy[i].second);
        if(xdist+ydist>t[i]-prevtm){
            ans=false;
            break;
        }else{
            int tmdiff=t[i]-prevtm-(xdist+ydist);
            if(tmdiff%2==1){
                ans=false;
                break;
            }else{
                prevtm=t[i];
                prevx=xy[i].first;
                prevy=xy[i].second;
            }
        }
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
