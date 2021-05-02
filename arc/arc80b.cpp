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
    int n;
    cin >> n;
    vector<int> a(n+1);
    for(int i=1;i<n+1;i++){
        cin >> a[i];
    }
    vector<vector<int>> ans(h, vector<int>(w,-1));
    int x=0,y=0;
    for(int i=1;i<n+1;i++){
        for(int j=0;j<a[i];j++){
            ans[y][x]=i;
            if(x%2==0){
                if(y+1<h) y++;
                else x++;
            }else{
                if(y-1>=0) y--;
                else x++;
            }
            // cout << "x: " << x << " y: " << y << endl;
        }
    }
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cout << ans[i][j] << " ";
        }
        cout << endl;
    }
    return 0;
}
