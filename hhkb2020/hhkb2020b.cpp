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

    vector<string> s(h);
    for(auto &x:s){
        cin >> x;
    }

    int ans=0;
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(i+1<h && s[i][j]=='.' && s[i+1][j]=='.'){
                ans++;
            }
            if(j+1<w && s[i][j]=='.' && s[i][j+1]=='.'){
                ans++;
            }
        }
    }

    cout << ans << endl;

    return 0;
}
