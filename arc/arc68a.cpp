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
    ll x;
    cin >> x;
    ll ans=x/11;
    ll rem=x%11;
    if(x>10){
        if(rem==0){
            cout << 2*ans << endl;
        }else if(rem>0 && rem<7){
            cout << 2*ans+1 << endl;
        }else if(rem>=7){
            cout << 2*ans+2 << endl;
        }
    }else{
        if(rem<7){
            cout << 1 << endl;
        }else if(rem>=7){
            cout << 2 << endl;
        }
    }
    return 0;
}
