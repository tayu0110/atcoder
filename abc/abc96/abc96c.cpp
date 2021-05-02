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
    vector<string> plane(h);
    for(auto &x:plane)cin >> x;

    queue<pii> nxt;
    vector<vector<bool>> check(h,vector<bool>(w,false));
    nxt.push(make_pair(0,0));
    check[0][0]=true;
    bool ans=true;
    while(!nxt.empty()){
        int x=nxt.front().first,y=nxt.front().second;
        nxt.pop();
        if(x+1<w && !check[y][x+1]){
            nxt.push(make_pair(x+1,y));
            check[y][x+1]=true;
        }
        if(y+1<h && !check[y+1][x]){
            nxt.push(make_pair(x,y+1));
            check[y+1][x]=true;
        }
        if(plane[y][x]=='.')continue;
        else{
            if(x+1<w && plane[y][x+1]=='#')continue;;
            if(x-1>=0 && plane[y][x-1]=='#')continue;
            if(y+1<h && plane[y+1][x]=='#')continue;
            if(y-1>=0 && plane[y-1][x]=='#')continue;
            ans=false;
            break;
        }
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
