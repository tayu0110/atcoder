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
#define INF (1LL<<62)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int n,m,k;
    cin >> n >> m >> k;

    vector<int> a(n),b(m);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    for(int i=0;i<m;i++){
        cin >> b[i];
    }

    ll ans=0,time=0;
    int ai=0,bi=0;
    while(time<=k){ 
        if(ai==n-1 && bi==m)
            break;       
        if(ai==n-1){
            time+=b.at(bi);
            if(time>k && )
            ans++;
        }else if(bi==m-1){
            time+=a.at(ai);
            ans++;
        }
    }

    if(k-time>)

    cout << ans << endl;

    return 0;
}
