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
    vector<int> w(n);
    vector<int> sum(n);
    for(int i=0;i<n;i++){
        cin >> w[i];
        if(i==0)sum[i]=w[i];
        else sum[i]=sum[i-1]+w[i];
    }
    int tot=sum[n-1];
    int ans=inf;
    for(int i=0;i<n-1;i++){
        int s1=sum[i],s2=tot-sum[i];
        ans=min(ans,abs(s1-s2));
    }
    cout << ans << endl;
    return 0;
}
