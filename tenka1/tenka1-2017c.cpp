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
    ll N;
    cin >> N;
    for(ll h=1;h<=3500;h++){
        for(ll n=1;n<=3500;n++){
            ll d=4*h*n-N*n-N*h;
            if(d<=0)continue;
            if(N*h*n%d!=0)continue;
            ll w=N*h*n/d;
            cout << h << " " << n << " " << w << endl;
            return 0;
        }
    }
    return 0;
}
