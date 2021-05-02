//ATC001-A:DFS(深さ優先探索)実装例
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

using namespace std;
using ll = long long;

int h,w;
pair<int,int> sp,gp;
vector<vector<bool>> reached(500,vector<bool>(500,false));

void dfs(int, int, vector<vector<char>>&);

int main(int argc,char* argv[]){
    cin >> h >>w;

    vector<vector<char>> maze(h,vector<char>(w));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cin >> maze.at(i).at(j);
            if(maze.at(i).at(j)=='s')
                sp=make_pair(i,j);
            if(maze.at(i).at(j)=='g')
                gp=make_pair(i,j);
        }
    }

    dfs(sp.first,sp.second,maze);

    if(reached.at(gp.first).at(gp.second))
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
    
    return 0;
}

void dfs(int y, int x, vector<vector<char>> &m){
    if(x>w-1 || x<0 || y>h-1 || y<0)
        return;
    if(reached.at(y).at(x))
        return;
    if(m.at(y).at(x)=='#')
        return;
    
    reached.at(y).at(x)=true;

    dfs(y-1,x,m);
    dfs(y+1,x,m);
    dfs(y,x-1,m);
    dfs(y,x+1,m);
}
