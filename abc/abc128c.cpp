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
    int n,m;
    cin >> n >> m;
    vector<int> k(m),p(m);
    vector<vector<int>> s(m, vector<int>(0));
    for(int i=0;i<m;i++){
        cin >> k[i];
        for(int j=0;j<k[i];j++){
            int ss;
            cin >> ss;
            ss--;
            s[i].push_back(ss);
        }
    }
    for(int i=0;i<m;i++){
        cin >> p[i];
    }
    int ans=0;
    for(ll bit = 0; bit < (1<<n); bit++){
        bool flag=true;
        for(int i=0;i<m;i++){
            int count=0;
            for(int j=0;j<k[i];j++){
                if(bit & (1<<s[i][j])) count++;
                // cout << "reached3" << endl;
            }
            if(count%2!=p[i]) flag=false;
            // cout << "reached2" << endl;
        }
        if(flag) ans++;
        // cout << "reached1" << endl;
    }
    cout << ans << endl;
    return 0;
}
