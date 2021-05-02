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
    int h,w,a,b;
    cin >> h >> w >> a >> b;
    vector<vector<int>> ans(h,vector<int>(w,0));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(i<h-b && j<a) ans[i][j]=1;
            if(i>=h-b && j>=a) ans[i][j]=1;
            cout << ans[i][j];
        }
        cout << endl;
    }
    return 0;
}
