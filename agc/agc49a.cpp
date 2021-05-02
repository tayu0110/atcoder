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

struct UnionFind{
    vector<int> par;

    UnionFind(int n){
        par = vector<int>(n, -1);
    }
    int root(int x){
        if(par[x] < 0) return x;
        return par[x] = root(par[x]);
    }
    bool merge(int x, int y){
        int rx = root(x);
        int ry = root(y);
        if(rx == ry) return false;
        if(par[rx] > par[ry]) swap(x, y);
        par[rx] += par[ry];
        par[ry] = rx;
        return true;
    }
    bool isSame(int x, int y){
        int rx = root(x);
        int ry = root(y);
        return rx == ry;
    }
    int size(int x){
        return -par[root(x)];
    }
};

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    cin >> n;
    vector<string> s(n);
    for(int i=0;i<n;i++){
        cin >> s[i];
    }
    UnionFind up(n),down(n);
    for(int i=0;i<n;i++){
        for(int j=0;j<n;j++){
            if(s[i][j]=='1'){
                if(i<j){
                    down.merge(i,j);
                }else{
                    up.merge(i,j);
                }
            }
        }
    }
    vector<vector<int>> graph(n, vector<int>(0));
    for(int i=0;i<n;i++){
        if(i!=up.root(i))graph[up.root(i)].push_back(i);
        if(i!=down.root(i))graph[down.root(i)].push_back(i);
    }
    
    return 0;
}
