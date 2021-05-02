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
    int white=0;
    for(int i=0;i<h;i++){
        cin >> s[i];
        for(int j=0;j<w;j++){
            if(s[i][j]=='.')white++;
        }
    }
    vector<vector<int>> mp(h, vector<int>(w,inf));
    mp[0][0]=1;
    vector<vector<bool>> ck(h, vector<bool>(w,false));
    ck[0][0]=true;
    queue<pii> nt;
    nt.push(make_pair(0,0));
    while(!nt.empty()){
        int i=nt.front().first,j=nt.front().second;
        nt.pop();
        if(i-1>=0 && !ck[i-1][j] && s[i-1][j]=='.'){
            nt.push(make_pair(i-1,j));
            mp[i-1][j]=mp[i][j]+1;
            ck[i-1][j]=true;
        }
        if(i+1<h && !ck[i+1][j] && s[i+1][j]=='.'){
            nt.push(make_pair(i+1,j));
            mp[i+1][j]=mp[i][j]+1;
            ck[i+1][j]=true;
        }
        if(j-1>=0 && !ck[i][j-1] && s[i][j-1]=='.'){
            nt.push(make_pair(i,j-1));
            mp[i][j-1]=mp[i][j]+1;
            ck[i][j-1]=true;
        }
        if(j+1<w && !ck[i][j+1] && s[i][j+1]=='.'){
            nt.push(make_pair(i,j+1));
            mp[i][j+1]=mp[i][j]+1;
            ck[i][j+1]=true;
        }
    }
    if(mp[h-1][w-1]==inf) cout << -1 << endl;
    else cout << white-mp[h-1][w-1] << endl;
    return 0;
}
