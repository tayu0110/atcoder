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
    vector<int> c(n-1),s(n-1),f(n-1);
    for(int i=0;i<n-1;i++){
        cin >> c[i] >> s[i] >> f[i];
    }
    vector<ll> t(n);
    for(int i=0;i<n-1;i++){
        t[i]=s[i];
        for(int j=i;j<n-1;j++){
            if(t[j]<s[j])t[j]=s[j];
            t[j]+=(t[j]%f[j]==0 ? 0 : f[j]-(t[j]%f[j]));
            t[j+1]=t[j]+c[j];
            // cout << "i: " << i << " j: " << j << " t: " << t[j] << endl;
        }
        cout << t[n-1] << endl;
    }
    cout << 0 << endl;
    return 0;
}
