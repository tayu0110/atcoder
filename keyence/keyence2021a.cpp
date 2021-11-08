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
    vector<ll> a(n),b(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
        if(i-1>=0 && a[i]<a[i-1]) a[i]=a[i-1];
    }
    for(int i=0;i<n;i++){
        cin >> b[i];
    }
    vector<ll> ans(n);
    for(int i=0;i<n;i++){
        if(i==0){
            ans[i]=a[i]*b[i];
        }else{
            ans[i]=max(a[i]*b[i], ans[i-1]);
            if(b[i]<b[i-1]) b[i]=b[i-1];
        }
    }
    for(int i=0;i<n;i++){
        cout << ans[i] << endl;
    }
    return 0;
}
