#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
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
    vector<int> par;            //親の番号

    UnionFind(int n) : par(n){
        for(int i=0; i<n; i++) par[i]=i;
    }

    int root(int x){
        if(par[x] == x) return x;
        return par[x] = root(par[x]);
    }

    void merge(int x, int y){
        int rx = root(x);
        int ry = root(y);
        if(rx == ry) return ;
        par[rx] = ry;
    }

    bool isSame(int x, int y){
        int rx = root(x);
        int ry = root(y);
        return rx == ry;
    }
};

int main(){
    int n,m;
    cin >> n >> m;

    UnionFind uf(n);
    for(int i=0;i<m;i++){
        int a,b;
        cin >> a >> b;
        a--;b--;
        uf.merge(a,b);
    }

    vector<int> groupnum(n+1,0);
    int maxmember=0;
    for(int i=0;i<n;i++){
        int num=++groupnum[uf.root(i)];
        if(num>maxmember)maxmember=num;
    }

    cout << maxmember << endl;
}


//Wrong Answer
// int root(int, vector<int>&);

// int main(int argc,char* argv[]){
//     int n,m;
//     cin >> n >> m;

//     vector<int> par(n+1,0);
//     for(int i=0;i<n+1;i++){
//         par[i]=i;
//     }

//     vector<pii> inputdatta(m);
//     for(int i=0;i<m;i++){
//         cin >> inputdatta[i].first >> inputdatta[i].second;
//     }

//     for(int i=0;i<m;i++){
//         int a=inputdatta[i].first,b=inputdatta[i].second;

//         int x=root(a,par),y=root(b,par);
//         if(x==y)continue;
//         else{
//             par[x]=y;
//         }
//         // if(par[a]==a){
//         //     par[a]=b;
//         // }else if(par[b]==b){
//         //     par[b]=a;
//         // }else{
//         //     int r=root(par[b],par);
//         //     par[r]=a;
//         // }

//     }

//     // for(int i=1;i<n+1;i++){
//     //     if(par[i]==i)continue;
//     //     par[i]=root(i,par);
//     // }

//     vector<int> groupmember(n+1,0);
//     int mostpeople=0;
//     for(int i=1;i<n+1;i++){
//         groupmember[par[i]]++;
//         if(groupmember[par[i]]>mostpeople){
//             mostpeople=groupmember[par[i]];
//         }
//     }

//     cout << mostpeople << endl;

//     return 0;
// }

// int root(int child, vector<int> &tree){
//     if(tree[child]==child){
//         return child;
//     }else{
//         return tree[child]=root(tree[child],tree);
//     }

// }
