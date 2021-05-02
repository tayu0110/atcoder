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
    ld n;
    ld a,b;
    cin >> n >> a >> b;
    vector<ld> s(n);
    ld ave=0;
    ld maxval=0,minval=INF;
    for(int i=0;i<n;i++){
        cin >> s[i];
        ave+=s[i];
        maxval=max(maxval, s[i]);
        minval=min(minval, s[i]);
    }
    ave/=n;
    ld p,q;
    if(maxval-minval==0){
        cout << -1 << endl;
        return 0;
    }
    p=b/(maxval-minval);
    q=a-p*ave;
    cout << p << " " << q << endl;
    return 0;
}
