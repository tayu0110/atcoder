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
    int n,x,y;
    cin >> n >> x >> y;
    x--;
    y--;
    vector<vector<int>> l(n, vector<int>(n,-1));
    for(int i=0;i<n;i++){
        for(int j=i+1;j<n;j++){
            if(j<=x)l[i][j]=j-i;
            else if(i<=x && j>=y)l[i][j]=x-i+1+j-y;
            else if(i>=y)l[i][j]=j-i;
            else if(i<=x && j>x && j<y)l[i][j]=x-i+min(j-x, y-j+1);
            else if(i>x && i<y && j>=y)l[i][j]=j-y+min(y-i, i-x+1);
            else l[i][j]=min(j-i, i-x+y-j+1);
        }
    }
    map<int,int> ans;
    for(int i=0;i<n;i++){
        for(int j=0;j<n;j++){
            int now=l[i][j];
            if(ans.find(now)==ans.end())ans.insert(make_pair(now, 1));
            else ans[now]++;
        }
    }
    for(int i=1;i<n;i++){
        if(ans.find(i)==ans.end())cout << 0 << endl;
        else cout << ans[i] << endl;
    }
    return 0;
}
