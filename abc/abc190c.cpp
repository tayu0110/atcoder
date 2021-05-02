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

struct Edge {
    int to;
    long long weight;
    Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n,m;
    cin >> n >> m;
    vector<pii> ab(m);
    for(int i=0;i<m;i++){
        cin >> ab[i].first >> ab[i].second;
        ab[i].first--;
        ab[i].second--;
    }
    int k;
    cin >> k;
    vector<vector<int>> cd(2, vector<int>(k));
    for(int i=0;i<k;i++){
        cin >> cd[0][i] >> cd[1][i];
        cd[0][i]--;
        cd[1][i]--;
    }
    ll ans=0;
    for(ll bit = 0; bit < (1<<k); bit++){
        vector<int> a(n,0);
        ll sum=0;
        for(int digits=0;digits<k;digits++){
            if(bit & (1<<digits)){
                a[cd[1][digits]]++;
            }else{
                a[cd[0][digits]]++;
            }
        }
        for(int i=0;i<m;i++){
            int x=ab[i].first,y=ab[i].second;
            if(a[x]!=0 && a[y]!=0) sum++;
        }
        ans = max(ans, sum);
    }
    cout << ans << endl;
    return 0;
}
