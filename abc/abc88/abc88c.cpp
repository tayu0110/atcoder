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

    vector<vector<int>> c(3,vector<int>(3,0));
    for(auto &x:c)for(auto &y:x)cin >> y;

    vector<int> a(3,0),b(3,0);
    bool ans=false;
    for(b[0]=0;b[0]<100;b[0]++){
        a[0]=c[0][0]-b[0];
        a[1]=c[0][1]-b[0];
        a[2]=c[0][2]-b[0];
        b[1]=c[1][0]-a[0];
        if(b[1]!=c[1][1]-a[1] || b[1]!=c[1][2]-a[2])continue;
        b[2]=c[2][0]-a[0];
        if(b[2]!=c[2][1]-a[1] || b[2]!=c[2][2]-a[2])continue;
        else{
            ans=true;
            break;
        }
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
