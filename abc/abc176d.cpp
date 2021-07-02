#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
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

int main(int argc,char* argv[]){
    int h,w;
    cin >> h >> w;

    int ch,cw;
    cin >> ch >> cw;

    int dh,dw;
    cin >> dh >> dw;

    vector<vector<char>> maze(h, vector<char>(w));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cin >> maze[i][j];
        }
    }

    vector<vector<ll>> check(h,vector<ll>(w,INF));
    check[ch-1][cw-1]=0;

    queue<pii> to,warp;
    set<pii> reached;
    to.push(make_pair(ch-1,cw-1));

    while(!to.empty() || !warp.empty()){
        if(to.empty()){
            if(reached.count(warp.front())){
                warp.pop();
                continue;
            }
            else{
                to.push(warp.front());
                warp.pop();
            }
        }
        if(reached.count(to.front())){
            to.pop();
            continue;
        }
        reached.insert(to.front());
        int x=to.front().second;
        int y=to.front().first;

        if(x-1>=0 && maze[y][x-1]=='.'){
            to.push(make_pair(y,x-1));
            check[y][x-1]=check[y][x];
        }
        if(x+1<w && maze[y][x+1]=='.'){
            to.push(make_pair(y,x+1));
            check[y][x+1]=check[y][x];
        }
        if(y-1>=0 && maze[y-1][x]=='.'){
            to.push(make_pair(y-1,x));
            check[y-1][x]=check[y][x];
        }
        if(y+1<h && maze[y+1][x]=='.'){
            to.push(make_pair(y+1,x));
            check[y+1][x]=check[y][x];
        }

        for(int i=0;i<=2;i++){
            for(int j=0;j<=2;j++){
                if(x-j>=0 && y-i>=0 && maze[y-i][x-j]=='.'){
                    if(check[y][x]+1<check[y-i][x-j]){
                        warp.push(make_pair(y-i,x-j));
                        check[y-i][x-j]=check[y][x]+1;
                    }
                }
            }
        }
        for(int i=1;i<=2;i++){
            for(int j=0;j<=2;j++){
                if(x-j>=0 && y+i<h && maze[y+i][x-j]=='.'){
                    if(check[y][x]+1<check[y+i][x-j]){
                        warp.push(make_pair(y+i,x-j));
                        check[y+i][x-j]=check[y][x]+1;
                    }
                }
            }
        }
        for(int i=0;i<=2;i++){
            for(int j=1;j<=2;j++){
                if(x+j<w && y-i>=0 && maze[y-i][x+j]=='.'){
                    if(check[y][x]+1<check[y-i][x+j]){
                        warp.push(make_pair(y-i,x+j));
                        check[y-i][x+j]=check[y][x]+1;
                    }
                }
            }
        }
        for(int i=1;i<=2;i++){
            for(int j=1;j<=2;j++)
            if(x+j<w && y+i<h && maze[y+i][x+j]=='.'){
                if(check[y][x]+1<check[y+i][x+j]){
                    check[y+i][x+j]=check[y][x]+1;
                    warp.push(make_pair(y+i,x+j));
                }
            }
        }
        to.pop();
    }

    ll ans=check[dh-1][dw-1];

    if(ans==INF) cout << -1 << endl;
    else cout << check[dh-1][dw-1] << endl;

    return 0;
}
