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
    ll k;
    cin >> k;
    vector<vector<ll>> city(n,vector<ll>(n,0));
    for(int i=0;i<n;i++){
        for(int j=0;j<n;j++){
            cin >> city[i][j];
        }
    }
    ll ans=0;
    vector<int> way(n-1);
    for(int i=0;i<n-1;i++)way[i]=i+1;
    do{ 
        ll time=0;
        time+=city[0][way[0]];
        for(int i=1;i<n-1;i++){
            int prev=way[i-1],now=way[i];
            time+=city[prev][now];
        }
        int last=way[n-2];
        time+=city[last][0];
        if(time==k)ans++;
    }while(next_permutation(way.begin(),way.end()));
    cout << ans << endl;

    return 0;
}
