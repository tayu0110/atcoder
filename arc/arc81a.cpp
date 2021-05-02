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
    cin >> n;
    vector<ll> a(n);
    for(auto &x:a){
        cin >> x;
    }
    sort(a.begin(), a.end());
    ll ans=1;
    int flag=0;
    for(int i=a.size()-1;i>=0;i--){
        if(a[i]==a[i-1]){
            ans*=a[i];
            i--;
            flag++;
        }
        if(flag==2)break;
    }

    if(!flag)cout << 0 << endl;
    else cout << ans << endl;

    return 0;
}
