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
    int n,m,x,y;
    cin >> n >> m >> x >> y;
    vector<int> xc(n),yc(m);
    for(int i=0;i<n;i++) cin >> xc[i];
    for(int i=0;i<m;i++) cin >> yc[i];
    sort(xc.begin(), xc.end());
    sort(yc.begin(), yc.end());
    bool flag=false;
    for(int i=x+1;i<y+1;i++){
        if(xc[n-1]<i && yc[0]>=i)flag=true;
    }
    if(flag) cout << "No War" << endl;
    else cout << "War" << endl;
    return 0;
}
