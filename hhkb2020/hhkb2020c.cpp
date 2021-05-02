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
    vector<int> p(n);
    int maxval=0;
    for(auto &x:p){
        cin >>x;
        maxval=max(maxval,x);
    }

    set<int> check;
    int minval=0;
    for(int i=0;i<n;i++){
        int ans;
        check.insert(p[i]);
        for(int j=minval;j<=maxval;j++){
            if(check.find(j)==check.end()){
                ans=j;
                minval=j;
                break;
            }
            if(j==maxval){
                ans=maxval+1;
                minval=maxval+1;
                break;
            }
        }
        cout << ans << endl;
    }

    return 0;
}
