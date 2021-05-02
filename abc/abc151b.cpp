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
    int n,k,m;
    cin >> n >> k >> m;
    vector<int> a(n-1);
    int sum =0;
    for(int i=0;i<n-1;i++){
        cin >> a[i];
        sum += a[i];
    }
    if(n*m-sum>k)cout << -1 << endl;
    else {
        if(n*m-sum<0) cout << 0 << endl;
        else cout << n*m-sum << endl;
    }
    return 0;
}
