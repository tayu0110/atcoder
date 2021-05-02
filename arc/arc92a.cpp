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
    vector<pii> ab(n),cd(n);
    for(int i=0;i<n;i++) cin >> ab[i].first >> ab[i].second;
    for(int i=0;i<n;i++) cin >> cd[i].first >> cd[i].second;
    sort(ab.begin(), ab.end());
    sort(cd.begin(), cd.end());
    set<pii> ck;
    int ans=0;
    for(int i=0;i<n;i++){
        int c=cd[i].first,d=cd[i].second;
        pii maxp=make_pair(-1,-1);
        for(int j=0;j<n;j++){
            int a=ab[j].first, b=ab[j].second;
            if(a>c) break;
            if(b>d) continue;
            if(b>maxp.second){
                if(ck.find(make_pair(a,b))==ck.end()){
                    maxp=ab[j];
                }else continue;
            }
        }
        if(maxp!=make_pair(-1, -1)){
            ans++;
            ck.insert(maxp);
        }
    }
    cout << ans << endl;
    return 0;
}
