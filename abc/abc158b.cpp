#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    ll n,a,b;
    cin >> n >> a >> b;

    if(a>=n){
        cout << n << endl;
        return 0;
    }

    ll ab=a+b;
    if(ab>=n){
        cout << a << endl;
        return 0;
    }

    ll asum=a*(n/ab);
    if(n-ab*(n/ab)>a)asum+=a;
    else asum+=(n-ab*(n/ab));

    cout << asum << endl;

    return 0;
}
