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

    int n,m;
    cin >> n >> m;
    vector<int> x(m);
    for(auto &y:x){
        cin >> y;
    }

    if(n>=m){
        cout << 0 << endl;
        return 0;
    }

    sort(x.begin(),x.end());
    vector<int> diff(m-1);
    for(int i=0;i<m-1;i++){
        diff[i]=x[i+1]-x[i];
    }

    sort(diff.begin(),diff.end());

    ll maxdist=x[m-1]-x[0];
    for(int i=m-2;i>=m-n;i--){
        maxdist-=diff[i];
    }

    cout << maxdist << endl;

    return 0;
}
