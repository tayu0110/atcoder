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

// template<class T>T GCD(T a, T b){if(a<b){T temp=a;a=b;b=temp;}T res=a%b;while(res!=0){a=b;b=res;res=a%b;}return b;}
// template<class T>T LCM(T a, T b){T res=GCD(a,b);return a/res*b;}

template<class T>T GCD(T a, T b){if(a<b){T temp=a;a=b;b=temp;}T res=a%b;while(res!=0){a=b;b=res;res=a%b;}return b;}
template<class T>T LCM(T a, T b){T res=GCD(a,b);return max(a,b)/res*min(a,b);}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    cin >> n;
    vector<ll> t(n);
    for(int i=0;i<n;i++){
        cin >> t[i];
    }
    ll ans=t[0];
    for(int i=1;i<n;i++){
        ans=LCM(ans,t[i]);
        // cout << "NOW: " << "i: " << i << " " << ans << endl;
    }

    // cout << LCM(1000000000000000000,1000000000000000000) << endl;

    cout << ans << endl;

    return 0;
}
