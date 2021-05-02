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

int c[10][10];
vector<ll> mag(10,inf);
void dfs(int &b, int i, ll cost, set<int> &ck){
    // cout << "b: " << b << " i: " << i << endl;
    if(i==1){
        mag[b]=min(mag[b],cost);
        return;
    }
    if(mag[i]!=inf){
        mag[b]=min(mag[b], cost+mag[i]);
        return;
    }
    for(int j=0;j<10;j++){
        if(ck.find(j)!=ck.end())continue;
        if(cost+c[i][j]>=mag[b])continue;
        ck.insert(j);
        dfs(b, j, cost+c[i][j], ck);
        ck.erase(j);
    }
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int h,w;
    cin >> h >> w;
    for(int i=0;i<10;i++){
        for(int j=0;j<10;j++){
            cin >> c[i][j];
        }
    }
    vector<vector<int>> a(h,vector<int>(w));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cin >> a[i][j];
        }
    }
    for(int i=0;i<10;i++){
        if(i==1)continue;
        set<int> ck;
        ck.insert(i);
        dfs(i, i, 0, ck);
        // cout << "i:" << i << endl;
    }
    ll ans=0;
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(a[i][j]==-1)continue;
            if(a[i][j]==1)continue;
            int now=a[i][j];
            ans+=mag[now];
        }
    }
    // for(int i=0;i<10;i++)cout << "mag[i]: " <<  mag[i] << endl;
    cout << ans << endl;
    return 0;
}
