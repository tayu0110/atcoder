#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
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

template<class T>T GCD(T a, T b){if(a<b){T temp=a;a=b;b=temp;}T res=a%b;while(res!=0){a=b;b=res;res=a%b;}return b;}
template<class T>T LCM(T a, T b){T res=GCD(a,b);return a*b/res;}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    ll a,b,c,d;
    cin >> a >> b >> c >> d;

    ll cdlcm=LCM(c,d);

    ll csum=b/c-(a-1)/c;
    ll dsum=b/d-(a-1)/d;
    ll cdsum=b/cdlcm-(a-1)/cdlcm;

    cout << (b-a+1)-(csum+dsum-cdsum) << endl;

    return 0;
}
