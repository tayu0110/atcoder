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
    int h,w;
    cin >> h >> w;
    vector<string> a(h);
    for(int i=0;i<h;i++){
        cin >> a[i];
    }
    vector<vector<int>> ans(h,vector<int>(w,inf));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(a[i][j]=='#') ans[i][j]=0;
            else {
                if(j-1>=0) ans[i][j]=min(ans[i][j], ans[i][j-1]+1);
            }
        }
    }
    for(int i=0;i<h;i++){
        for(int j=w-1;j>=0;j--){
            if(a[i][j]=='#') continue;
            else {
                if(j+1<w) ans[i][j]=min(ans[i][j], ans[i][j+1]+1);
            }
        }
    }
    for(int i=0;i<w;i++){
        for(int j=0;j<h;j++){
            if(a[j][i]=='#') continue;
            else {
                if(j-1>=0) ans[j][i]=min(ans[j][i], ans[j-1][i]+1);
            }
        }
    }
    for(int i=0;i<w;i++){
        for(int j=h-1;j>=0;j--){
            if(a[j][i]=='#') continue;
            else {
                if(j+1<h) ans[j][i]=min(ans[j][i], ans[j+1][i]+1);
            }
        }
    }
    int res=0;
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            res=max(res, ans[i][j]);
        }
    }
    cout << res << endl;
    return 0;
}
