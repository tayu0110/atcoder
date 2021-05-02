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

struct UnionFind{
    vector<int> par;

    UnionFind(int n){
        for(int i=0;i<n;i++){
            par.push_back(i);    
        }
    }
    int root(int x){
        if(par[x] == x) return x;
        return par[x] = root(par[x]);
    }
    bool merge(int x, int y){
        int rx = root(x);
        int ry = root(y);
        if(rx == ry) return false;
        // if(par[rx] > par[ry]) swap(x, y);
        // par[rx] += par[ry];
        par[ry] = rx;
        return true;
    }
    bool isSame(int x, int y){
        int rx = root(x);
        int ry = root(y);
        return rx == ry;
    }
    // int size(int x){
    //     return -par[root(x)];
    // }
};

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n,m;
    cin >> n >> m;

    UnionFind city(n);
    for(int i=0;i<m;i++){
        int a,b;
        cin >> a >> b;
        city.merge(a,b);
    }

    set<int> check;
    for(int i=0;i<n;i++){
        int j=city.root(i);
        check.insert(j);
    }

    cout << check.size()-1 << endl;

    return 0;
}
