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

    ll x,y;
    cin >> x >> y;

    ll a,b,c,d;
    ll nx=-x,ny=-y;
    a=(x<=y?abs(y-x):INF);
    b=(nx<=y?abs(y-nx):INF)+1;
    c=(x<=ny?abs(ny-x):INF)+1;
    d=(nx<=ny?abs(nx-ny):INF)+2;
    cout << min(a, min(b, min(c, d))) << endl;

    // if(abs(x)==abs(y) && x!=y){
    //     cout << 1 << endl;
    //     return 0;
    // }
    // if(x==0){
    //     if(y==0)cout << 0 << endl;
    //     else if(y<0)cout << abs(y)+1 << endl;
    //     else if(y>0)cout << y << endl;
    //     return 0;
    // }
    // if(y==0){
    //     if(x<0)cout << abs(x) << endl;
    //     else if(x>0)cout << x+1 << endl;
    //     return 0;
    // }
    // if(x<0 && y<0){
    //     if(x>y)cout << x-y+2 << endl;
    //     else cout << y-x << endl;        
    // }else if(x<0 && y>0){
    //     if(abs(x)>y)cout << abs(x)-y+1 << endl;
    //     else cout << y-abs(x)+1 << endl;
    // }else if(x>0 && y<0){
    //     if(x>abs(y))cout << x-abs(y)+1 << endl;
    //     else cout << abs(y)-x+1 << endl;
    // }else if(x>0 && y>0){
    //     if(x>y)cout << x-y+1 << endl;
    //     else cout << y-x << endl;
    // }

    return 0;
}
