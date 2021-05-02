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
    vector<ll> h(n),s(n);
    for(int i=0;i<n;i++){
        cin >> h[i] >> s[i];
    }
    ll l=0,r=INF;
    while(r-l>1){
        ll m=(l+r)/2;
        bool flag=true;
        vector<ll> t(n);
        for(int i=0;i<n;i++){
            if(m<h[i]) flag=false;
            else t[i]=(m-h[i])/s[i];
        }
        sort(t.begin(),t.end());
        for(int i=0;i<n;i++){
            if(t[i]<i) flag=false;
        }
        if(flag) r=m;
        else l=m;
    }
    cout << r << endl;
    return 0;
}
