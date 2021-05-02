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
    string s, t;
    cin >> s >> t;
    vector<int> d(0);
    for(int i=0;i<n;i++){
        if(i+1<n && s[i]==s[i+1]){
            d.push_back(2);
            i++;
        }else{
            d.push_back(1);
        }
    }
    ll ans=1;
    for(int i=0;i<d.size();i++){
        if(i-1<0 && d[i]==1){
            ans*=3;
        }else if(i-1<0 && d[i]==2){
            ans*=6;
        }else if(d[i]==1 && d[i-1]==1){
            ans*=2;
            ans%=MOD;
        }else if(d[i]==1 && d[i-1]==2){
            continue;
        }else if(d[i]==2 && d[i-1]==1){
            ans*=2;
            ans%=MOD;
        }else if(d[i]==2 && d[i-1]==2){
            ans*=3;
            ans%=MOD;
        }
    }
    cout << ans << endl;
    return 0;
}
