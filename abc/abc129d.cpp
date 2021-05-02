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
    vector<string> s(h);
    for(int i=0;i<h;i++){
        cin >> s[i];
    }
    vector<vector<int>> ver(h,vector<int>(w)),hor(h,vector<int>(w));
    for(int i=0;i<h;i++)for(int j=0;j<w;j++){
        if(s[i][j]=='#')continue;
        if(j==0) hor[i][j]=1;
        else hor[i][j]=hor[i][j-1]+1;
        if(i==0) ver[i][j]=1;
        else ver[i][j]=ver[i-1][j]+1;
    }
    for(int i=h-1;i>=0;i--)for(int j=w-1;j>=0;j--){
        if(s[i][j]=='#')continue;
        if(j+1<w && s[i][j+1]!='#') hor[i][j]=hor[i][j+1];
        if(i+1<h && s[i+1][j]!='#') ver[i][j]=ver[i+1][j];
    }
    int ans=0;
    for(int i=0;i<h;i++)for(int j=0;j<w;j++){
        if(s[i][j]=='#') continue;
        ans=max(ans, ver[i][j]+hor[i][j]-1);
    }
    cout << ans << endl;
    return 0;
}
