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
    int n,k;
    cin >> n >> k;
    string s;
    cin >> s;
    int count=0,h=0;
    if(s[0]=='L')count++;
    if(s[n-1]=='R')count++;
    for(int i=0;i<n;i++){
        if(i+1<s.size() && s[i]=='R' && s[i+1]=='L')count++;
        if(i+1<s.size() && s[i]=='R' && s[i+1]=='R')h++;
        if(i-1>=0 && s[i]=='L' && s[i-1]=='L')h++;
    }
    // cout << count << endl;
    if(count<k)cout << n-1 << endl;
    else cout << min(n-1, h+k*2) << endl;
    return 0;
}
