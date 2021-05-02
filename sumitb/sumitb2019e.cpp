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
    vector<ll> a(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    if(a[0]!=0){
        cout << 0 << endl;
        return 0;
    }
    vector<ll> ans(n);
    vector<vector<ll>> rgb(3, vector<ll>(n,0));
    ans[0]=3;
    rgb[0][0]=1;
    for(int i=1;i<n;i++){
        ll c=0;
        bool flag=true;
        for(int j=0;j<3;j++){
            if(rgb[j][i-1]==a[i]){
                c++;
                if(flag){
                    rgb[j][i]=rgb[j][i-1]+1;
                    flag=false;
                }else rgb[j][i]=rgb[j][i-1];
            }else{
                rgb[j][i]=rgb[j][i-1];
            }
        }
        if(c==0){
            cout << 0 << endl;
            return 0;
        }
        ans[i]=ans[i-1]*c%MOD;
    }
    cout << ans[n-1]%MOD << endl;
    return 0;
}
