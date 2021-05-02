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
    int x1,y1,x2,y2;
    cin >> x1 >> y1 >> x2 >> y2;

    int x3,y3,x4,y4;
    int x,y;
    x=x2-x1;
    y=y2-y1;

    x3=x2-y;
    y3=y2+x;
    x4=x1-y;
    y4=y1+x;

    cout << x3 << " " << y3 << " " << x4 << " " << y4;

    return 0;
}
