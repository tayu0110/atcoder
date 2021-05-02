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
    string s;
    cin >> s;
    int n=s.size();
    vector<int> ans(n,1);
    for(int i=0;i<n-2;i++){
        if(s[i]!='R')continue;
        if(s[i+1]=='L')continue;
        ans[i+2]+=ans[i];
        ans[i]=0;
    }
    for(int i=n-1;i>=2;i--){
        if(s[i]!='L')continue;
        if(s[i-1]=='R')continue;
        ans[i-2]+=ans[i];
        ans[i]=0;
    }
    for(int i=0;i<n;i++){
        cout << ans[i] << " ";
    }
    return 0;
}
