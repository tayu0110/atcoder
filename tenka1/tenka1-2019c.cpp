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
    string s;
    cin >> s;
    vector<int> w(n+1),b(n+1);
    w[0]=0;
    b[n]=0;
    for(int i=0;i<n;i++){
        if(s[i]=='#')w[i+1]=w[i]+1;
        else w[i+1]=w[i];
    }
    for(int i=n-1;i>=0;i--){
        if(s[i]=='.')b[i]=b[i+1]+1;
        else b[i]=b[i+1];
    }
    int minval=inf;
    for(int i=0;i<n+1;i++){
        minval=min(minval, w[i]+b[i]);
    }
    cout << minval << endl;
    return 0;
}
