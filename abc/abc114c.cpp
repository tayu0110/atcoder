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

ll ans=0;
ll digits(ll n){
    int exp=0;
    while(n>0){
        n/=10;
        exp++;
    }
    return exp;
}
int num_count(string num, char c) {
    return count(num.begin(), num.end(), c);
}
void dfs(ll &n, int &d, string num){
    if(num.length()>d) return;
    if(num_count(num, '3')!=0 && num_count(num, '5')!=0 && num_count(num, '7')!=0){
        if(stoll(num) <= n) ans++;
        else return;
    }
    dfs(n,d,num+"3");
    dfs(n,d,num+"5");
    dfs(n,d,num+"7");
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll n;
    cin >> n;
    int d=digits(n);
    dfs(n,d,"");
    cout << ans << endl;
    return 0;
}
