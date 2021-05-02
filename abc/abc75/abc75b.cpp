#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
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
    vector<string> maze(h);
    for(auto &x:maze)cin >> x;

    queue<pii> nxt;
    vector<vector<bool>> check(h,vector<bool>(w,false));
    nxt.push(make_pair(0,0));
    while(!nxt.empty()){
        int x=nxt.front().first,y=nxt.front().second;
        nxt.pop();
        if(check[y][x])continue;
        check[y][x]=true;
        if(x+1<w){
            if(!check[y][x+1])nxt.push(make_pair(x+1,y));
        }
        if(y+1<h){
            if(!check[y+1][x])nxt.push(make_pair(x,y+1));
        }
        if(maze[y][x]=='#'){
            continue;
        }
        int bomb=0;
        for(int i=-1;i<2;i++){
            if(x+i<0 || x+i>=w)continue;
            for(int j=-1;j<2;j++){
                if(i==0 && j==0)continue;
                if(y+j<0 || y+j>=h)continue;
                if(maze[y+j][x+i]=='#')bomb++;
            }
        }
        maze[y][x]=bomb+'0';
    }

    for(auto &x:maze)cout << x << endl;

    return 0;
}
