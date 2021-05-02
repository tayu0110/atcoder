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
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    int n, m, x;
    cin >> n >> m >> x;

    set<int> a;
    vector<int> maze(n+1,0);
    for(int i=0;i<m;i++){
        int y;
        cin >> y;
        a.insert(y);
    }

    for(int i=1;i<n+1;i++){
        maze[i]=maze[i]+maze[i-1];
        if(a.count(i))maze[i]++;
    }

    int ans = min(maze[x],maze[n]-maze[x]);

    cout << ans << endl;

    return 0;
}
