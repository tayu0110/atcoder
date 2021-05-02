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
    ll n,k;
    cin >> n >> k;
    if(k%2==0){
        ll l2=0,l0=0;
        for(int i=1;i<n+1;i++){
            if(i%k==0)l0++;
            if(i%k==k/2)l2++;
        }
        cout << l0*l0*l0 + l2*l2*l2 << endl;
    }else{
        ll l=0;
        for(int i=1;i<n+1;i++){
            if(i%k==0)l++;
        }
        cout << l*l*l << endl;
    }
    return 0;
}
