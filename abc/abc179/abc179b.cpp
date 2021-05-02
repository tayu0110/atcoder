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

    vector<pii> d(n);
    for(auto &x:d){
        cin >> x.first >> x.second;
    }

    bool ans=false;
    for(int i=0;i<n-2;i++){
        if(d[i].first==d[i].second && d[i+1].first==d[i+1].second && d[i+2].first==d[i+2].second)
            ans=true;
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
