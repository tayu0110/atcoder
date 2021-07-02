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

    int n;
    cin >> n;
    vector<ll> d(n);
    for(auto &x:d){
        cin >> x;
    }
    int m;
    cin >> m;
    vector<ll> t(m);
    for(auto &x:t){
        cin >> x;
    }

    if(n<m){
        cout << "NO" << endl;
        return 0;
    }

    sort(d.begin(),d.end());
    sort(t.begin(),t.end());

    bool ans=true;
    int ncount=0,mcount=0;
    while(mcount<m){
        for(int i=ncount;i<n;i++){
            if(d[i]==t[mcount]){
                ncount=i+1;
                break;
            }else{
                if(i==n-1){
                    ncount=n;
                    ans=false;
                }
            }
        }
        mcount++;
        if(n-ncount==0){
            break;
        }
        if(n-ncount<m-mcount){
            ans=false;
            break;
        }
    }

    if(ans) cout << "YES" << endl;
    else cout << "NO" << endl;

    return 0;
}
