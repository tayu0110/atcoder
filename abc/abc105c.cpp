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

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    ll n;
    cin >> n;
    if(n==0){
        cout << 0 << endl;
        return 0;
    }
    vector<int> ans(0);
    while(abs(n)>0){
        ll k=n/(-2);
        // cout << "k: " << k << " n: " << n << endl;
        if(n-(-2)*k==1){
            // cout << "push: " << 1 << endl;
            ans.push_back(1);
            n=k;
        }else if(n-(-2)*k==-1){
            // cout << "push: " << 1 << endl;
            ans.push_back(1);
            if(n-(-2)*(k+1)==1) n=k+1;
            else if(n-(-2)*(k-1)==1) n=k-1;
        }else{
            // cout << "push: " << 0 << endl;
            ans.push_back(0);
            n=k;
        }
    }
    for(int i=ans.size()-1;i>=0;i--){
        cout << ans[i];
    }
    cout << endl;
    return 0;
}
