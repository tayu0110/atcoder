#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
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

int main(int argc,char* argv[]){
    int n,x,t;
    cin >> n >> x >> t;

    int ans=0;
    if(n%x==0) ans=t*(n/x);
    else ans=t*((n/x)+1);

    cout << ans << endl;

    return 0;
}
