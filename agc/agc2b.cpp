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
    int n,m;
    cin >> n >> m;
    vector<pii> xy(m);
    for(int i=0;i<m;i++){
        int x,y;
        cin >> x >> y;
        x--;y--;
        xy[i].first=x;
        xy[i].second=y;
    }

    int ans=1;
    vector<int> box(n,1);
    vector<bool> red(n,false);
    red[0]=true;
    for(int i=0;i<m;i++){
        int x=xy[i].first,y=xy[i].second;
        if(red[x]){
            if(red[y]==false)ans++;
            red[y]=true;
            if(box[x]==1){
                ans--;
                red[x]=false;
            }
        }
        box[x]--;
        box[y]++;
    }
    cout << ans << endl;
    return 0;
}
