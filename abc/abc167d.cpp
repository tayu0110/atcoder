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
    ll k;
    cin >> n >> k;
    vector<int> a(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
        a[i]--;
    }
    map<int, int> mp;
    int now=0;
    mp.insert(make_pair(0,0));
    ll i=1;
    vector<int> l(1,0);
    for(;i<=k;i++){
        now=a[now];
        if(mp.find(now)!=mp.end()) break;
        mp.insert(make_pair(now,i));
        l.push_back(now);
    }
    ll prev=mp[now];
    k-=prev;
    k%=(i-prev);
    cout << l[prev+k]+1 << endl;
    return 0;
}
