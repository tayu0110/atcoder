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

    const ll mod=998244353;

    int n,k;
    cin >> n >> k;
    set<pii> check;
    for(int i=0;i<k;i++){
        int l,r;
        cin >> l >> r;
        check.insert(make_pair(l,r));
    }

    vector<ll> point(n,0);
    point[0]=1;
    for(int i=1;i<n;i++){
        for(int j=0;j<i;j++){
            if(point[j]==0){
                continue;
            }
            int diff=i-j;
            for(auto it=check.begin();it!=check.end();it++){
                if(diff>=it->first && diff<=it->second){
                    point[i]+=point[j];
                    break;
                }else if(diff<it->first){
                    break;
                }
            }
        }
    }

    cout << point[n-1]%mod << endl;

    return 0;
}
