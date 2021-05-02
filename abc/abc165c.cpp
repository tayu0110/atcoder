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

int ans=0;
int n,m,q;
int a[51],b[51],c[51],d[51];
int seq[12];
void dfs(int idx){
    if(idx==n+1){
        int count=0;
        // for(int i=1;i<n+1;i++) cout << seq[i] << " ";
        for(int i=0;i<q;i++){
            if(seq[b[i]]-seq[a[i]]==c[i])count+=d[i];
        }
        ans=max(count,ans);
        // cout << "ans: " << ans << endl;
        return;
    }else{
        for(int i=seq[idx-1];i<m+1;i++){
            seq[idx]=i;
            dfs(idx+1);
        }
        return;
    }
}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    cin >> n >> m >> q;
    for(int i=0;i<q;i++){
        cin >> a[i] >> b[i] >> c[i] >> d[i];
    }
    seq[0]=1;
    dfs(1);
    cout << ans << endl;
    return 0;
}
