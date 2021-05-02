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

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    ll t;
    cin >> n >> t;
    vector<ll> tt(n);
    vector<ll> tm(n-1);
    for(int i=0;i<n;i++){
        cin >> tt[i];
        if(i!=0)tm[i-1]=tt[i]-tt[i-1];
    }

    ll sum=0;
    for(int i=0;i<tm.size();i++){
        if(tm[i]>t)sum+=t;
        else sum+=tm[i];
    }

    cout << sum+t << endl;

    return 0;
}
