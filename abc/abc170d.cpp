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
    int n;
    cin >> n;
    vector<int> a(n);
    map<int, int> mp;
    for(int i=0;i<n;i++){
        cin >> a[i];
        mp[a[i]]++;
    }
    sort(a.begin(), a.end());
    set<int> ck;
    int ans=0;
    for(int i=0;i<n;i++){
        int k=a[i];
        if(mp[k]>1) mp[k]=0;
        bool flag=true;
        for(int j=1;j*j<=k;j++){
            if(k%j!=0) continue;
            if(ck.find(j)!=ck.end() || ck.find(k/j)!=ck.end()) {
                flag=false;
                break;
            }
        }
        if(flag) {
            ans+=mp[k];
            ck.insert(k);
        }
    }
    cout << ans << endl;
    return 0;
}
