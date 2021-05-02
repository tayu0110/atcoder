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
    string s;
    cin >> s;
    vector<int> e(n),w(n);
    e[0]=(s[0]=='E' ? 1 : 0);
    w[n-1]=(s[n-1]=='W' ? 1 : 0);
    for(int i=1;i<n;i++){
        if(s[i]=='E')e[i]+=e[i-1]+1;
        else e[i]=e[i-1];
    }
    for(int i=n-2;i>=0;i--){
        if(s[i]=='W')w[i]+=w[i+1]+1;
        else w[i]=w[i+1];
    }
    int ans=inf;
    for(int i=0;i<n;i++){
        if(i==0){
            ans=min(ans, n-1-w[i+1]);
        }else if(i==n-1){
            ans=min(ans, n-1-e[i-1]);
        }else{
            ans=min(ans, n-1-e[i-1]-w[i+1]);
        }
    }
    cout << ans << endl;
    return 0;
}
