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
    int ans=0;
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(s[i][j]!='.')continue;
            vector<vector<int>> m(h,vector<int>(w,0));
            vector<vector<bool>> ck(h,vector<bool>(w,false));
            queue<pii> nt;
            nt.push(make_pair(i,j));
            ck[i][j]=true;
            while(!nt.empty()){
                int y=nt.front().first,x=nt.front().second;
                nt.pop();
                if(y+1<h && s[y+1][x]=='.' && !ck[y+1][x]){
                    nt.push(make_pair(y+1,x));
                    ck[y+1][x]=true;
                    m[y+1][x]=m[y][x]+1;
                    ans=max(ans, m[y+1][x]);
                }
                if(y-1>=0 && s[y-1][x]=='.' && !ck[y-1][x]){
                    nt.push(make_pair(y-1,x));
                    ck[y-1][x]=true;
                    m[y-1][x]=m[y][x]+1;
                    ans=max(ans, m[y-1][x]);
                }
                if(x+1<w && s[y][x+1]=='.' && !ck[y][x+1]){
                    nt.push(make_pair(y,x+1));
                    ck[y][x+1]=true;
                    m[y][x+1]=m[y][x]+1;
                    ans=max(ans, m[y][x+1]);
                }
                if(x-1>=0 && s[y][x-1]=='.' && !ck[y][x-1]){
                    nt.push(make_pair(y,x-1));
                    ck[y][x-1]=true;
                    m[y][x-1]=m[y][x]+1;
                    ans=max(ans, m[y][x-1]);
                }
            }
        }
    }
    cout << ans << endl;
    return 0;
}
