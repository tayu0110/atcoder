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
#define inf (1<<29)             //1<<31でオーバーフロー

int main(int argc,char* argv[]){
    int n,m;
    cin >> n >> m;

    // const int maxvar=100000000;

    int a[m],b[m],c[m],dist[100][100];
    for(int i=0;i<m;i++){
        cin >> a[i] >> b[i] >> c[i];
        a[i]--;
        b[i]--;
    }

    for(int i=0;i<n;i++){
        for(int j=0;j<n;j++){
            if(i==j)dist[i][j]=0;
            else dist[i][j]=inf;
        }
    }

    for(int i=0;i<m;i++){
        int ai=a[i], bi=b[i], ci=c[i];
        dist[ai][bi]=min(dist[ai][bi],ci);
        dist[bi][ai]=min(dist[bi][ai],ci);
    }

    for(int k=0;k<n;k++){
        for(int i=0;i<n;i++){
            for(int j=0;j<n;j++){
                dist[i][j]=min(dist[i][j],dist[i][k]+dist[k][j]);
            }
        }
    }

    int ans=m;
    for(int i=0;i<m;i++){
        bool shortest=false;
        int ai=a[i],bi=b[i],ci=c[i];
        for(int j=0;j<n;j++)
            if(dist[j][ai]+ci==dist[j][bi]) shortest=true;
        if(shortest) ans=ans-1;
    }

    cout << ans << endl;

    return 0;
}
