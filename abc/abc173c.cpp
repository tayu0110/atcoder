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
    int h,w,k;
    cin >> h >> w >> k;
    vector<string> c(h);
    for(int i=0;i<h;i++){
        cin >> c[i];
    }
    int ans=0;
    for(int i=0;i<(1<<h);i++){
        for(int j=0;j<(1<<w);j++){
            int count=0;
            for(int m=0;m<h;m++){
                if(i & (1<<m)) continue;
                for(int l=0;l<w;l++){
                    if(j & (1<<l)) continue;
                    if(c[m][l]=='#') count++;
                    // cout << "reached" << endl;
                }
            }
            // cout << count << endl;
            if(count==k) ans++;
        }
    }
    cout << ans << endl;
    return 0;
}
