//queueを利用した幅優先探索の実装＆迷路の最短経路問題解決
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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    int r,c;
    cin >> r >> c;

    pii sp,gp;
    cin >> sp.first >> sp.second >> gp.first >> gp.second;

    vector<vector<char>> maze(r,vector<char>(c));
    for(int i=0;i<r;i++){
        for(int j=0;j<c;j++)
            cin >> maze[i][j];
    }

    queue<pii> nxtstep;
    vector<vector<int>> step(r,vector<int>(c,-1));
    
    nxtstep.push(make_pair(sp.first-1,sp.second-1));
    step[sp.first-1][sp.second-1]=0;

    while(!(nxtstep.empty())){
        pii v=nxtstep.front();
        int vy=v.first;
        int vx=v.second;

        if(maze[vy+1][vx]!='#' && step[vy+1][vx]==-1){
            step[vy+1][vx]=step[vy][vx]+1;
            nxtstep.push(make_pair(vy+1,vx));
        }
        if(maze[vy-1][vx]!='#' && step[vy-1][vx]==-1){
            step[vy-1][vx]=step[vy][vx]+1;
            nxtstep.push(make_pair(vy-1,vx));
        }
        if(maze[vy][vx+1]!='#' && step[vy][vx+1]==-1){
            step[vy][vx+1]=step[vy][vx]+1;
            nxtstep.push(make_pair(vy,vx+1));
        }
        if(maze[vy][vx-1]!='#' && step[vy][vx-1]==-1){
            step[vy][vx-1]=step[vy][vx]+1;
            nxtstep.push(make_pair(vy,vx-1));
        }

        nxtstep.pop();
    }

    cout << step[gp.first-1][gp.second-1] << endl;

    return 0;
}
