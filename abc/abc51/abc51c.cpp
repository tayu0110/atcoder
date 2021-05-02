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
    int sx, sy, tx, ty;
    cin >> sx >> sy >> tx >> ty;

    string ans;

    int dx=tx-sx, dy=ty-sy;

    for(int i=0;i<dx;i++){
        ans.push_back('R');
    }
    for(int i=0;i<dy;i++){
        ans.push_back('U');
    }
    for(int i=0;i<dx;i++){
        ans.push_back('L');
    }
    for(int i=0;i<dy;i++){
        ans.push_back('D');
    }
    ans.push_back('L');
    for(int i=0;i<dy+1;i++){
        ans.push_back('U');
    }
    for(int i=0;i<dx+1;i++){
        ans.push_back('R');
    }
    ans.push_back('D');
    ans.push_back('R');
    for(int i=0;i<dy+1;i++){
        ans.push_back('D');
    }
    for(int i=0;i<dx+1;i++){
        ans.push_back('L');
    }
    ans.push_back('U');

    cout << ans << endl;

    return 0;
}
