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

    int s;
    cin >> s;

    if(s<3){
        cout << 0 << endl;
        return 0;
    }

    int n=s/3,exp=0;
    ll ans=0;
    for(int i=0;i<n;i++){
        int N=n-i;
        exp=s-3*N;
        ll prog=0;
        
        ans+=prog;
        ans%=MOD;
    }

    cout << ans << endl;

    return 0;
}
