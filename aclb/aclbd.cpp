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

    int n,k;
    cin >> n >> k;

    vector<int> a(n);
    for(auto &x:a){
        cin >> x;
    }

    vector<int> dp(n,0);
    vector<vector<int>> maxque(2,vector<int>(0));
    dp[0]=1;
    int maxval=1;
    for(int i=1;i<n;i++){
        int count=maxque.size()-1;
        for(int j=i-1;j>=0;j--){
            if(abs(a[i]-a[j])<=k){
                dp[i]=max(dp[i],dp[j]+1);
            }else{

            }
        }
        if(maxval<dp[i]){
            maxval=dp[i];
            maxque[0].push_back(maxval);
        }
    }

    cout << maxval << endl;

    return 0;
}
