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

struct Edge {
    int to;
    long long weight;
    Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

ll modPow(long long a, long long n, long long p) {
    if (n == 0) return 1;
    if (n == 1) return a % p;
    if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
    long long t = modPow(a, n / 2, p);
    return (t * t) % p;
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    string s;
    cin >> s;
    int n = s.size();
    vector<int> m(n+1);
    for(int i = n-1; i >= 0; i--) {
        int a=s[i]-'0';
        m[i]=(a*modPow(10,n-i-1,2019)+m[i+1])%2019;
        // cout << m[i] << endl;
    }
    map<int, ll> mp;
    for(int i = 0; i < n+1; i++) {
        mp[m[i]]++;
    }
    ll ans=0;
    for(auto it=mp.begin();it!=mp.end();it++){
        ll k=it->second;
        ans+=k*(k-1)/2;
    }
    cout << ans << endl;
    return 0;
}
