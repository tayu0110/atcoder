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
    int n;
    cin >> n;
    vector<int> a(n),b(n),c(n);
    for(int i=0;i<n;i++){
        cin >> a[i] >> b[i] >> c[i];
    }
    vector<vector<ll>> v(3,vector<ll>(n+1));
    for(int i=1;i<n+1;i++){
        v[0][i]=max(v[1][i-1]+a[i-1], v[2][i-1]+a[i-1]);
        v[1][i]=max(v[0][i-1]+b[i-1], v[2][i-1]+b[i-1]);
        v[2][i]=max(v[0][i-1]+c[i-1], v[1][i-1]+c[i-1]);
    }
    cout << max(v[0][n], max(v[1][n], v[2][n])) << endl;
    return 0;
}
