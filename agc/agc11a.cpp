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
    ll c,k;
    cin >> n >> c >> k;
    vector<ll> t(n);
    for(auto &x:t){
        cin >> x;
    }

    sort(t.begin(), t.end());

    ll count=1,bus=1,tm=t[0];
    for(int i=1;i<n;i++){
        if(t[i]<=tm+k){
            count++;
            if(count>c){
                bus++;
                count=1;
                tm=t[i];
            }
        }else{
            bus++;
            count=1;
            tm=t[i];
        }
    }

    cout << bus << endl;

    return 0;
}
