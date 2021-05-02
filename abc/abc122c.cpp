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
    int n,q;
    string s;
    cin >> n >> q >> s;
    vector<int> ac(n);
    ac[0]=0;
    for(int i=1;i<n;i++){
        if(s[i]=='C' && s[i-1]=='A') ac[i]=ac[i-1]+1;
        else ac[i]=ac[i-1];
    }
    vector<int> ans(q);
    for(int i=0;i<q;i++){
        int l,r;
        cin >> l >> r;
        l--;r--;
        ans[i]=ac[r]-ac[l];
    }
    for(int i=0;i<q;i++){
        cout << ans[i] << endl;
    }
    return 0;
}
