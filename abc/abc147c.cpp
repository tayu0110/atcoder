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
    int n;
    cin >> n;
    vector<int> a(n);
    vector<vector<int>> x(n,vector<int>(0)),y(n,vector<int>(0));
    for(int i=0;i<n;i++){
        cin >> a[i];
        for(int j=0;j<a[i];j++){
            int k,l;
            cin >> k >> l;
            k--;
            x[i].push_back(k);
            y[i].push_back(l);
        }
    }

    int ans=0;
    for(ll bit = 0; bit < (1<<n); bit++){
        bool flag=true;
        for(int i=0;i<n;i++){
            if(!(bit & (1<<i)))continue;
            for(int j=0;j<a[i];j++){
                if(((bit>>x[i][j]) & 1) ^ y[i][j]){
                    flag=false;
                }
            }
        }
        if(flag){
            int count=0;
            for(int i=0;i<n;i++){
                if(bit & (1<<i))count++;
            }
            ans=max(ans, count);
        }
    }
    cout << ans << endl;
    return 0;
}
