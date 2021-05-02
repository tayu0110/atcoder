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
    vector<pii> xy(n);
    for(int i=0;i<n;i++){
        cin >> xy[i].first >> xy[i].second;
    }
    int ans=0;
    for(int i=0;i<n;i++){
        int x1=xy[i].first,y1=xy[i].second;
        for(int j=i+1;j<n;j++){
            int x2=xy[j].first,y2=xy[j].second;
            if(abs(y2-y1)<=abs(x2-x1)) ans++;
        }
    }
    cout << ans << endl;
    return 0;
}
