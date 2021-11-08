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
    vector<int> a(n),num(n+1,0);
    set<int> check;
    for(auto &x:a){
        cin >> x;
        check.insert(x);
        num[x]++;
    }

    int kind=check.size();
    if(kind<=k){
        cout << 0 << endl;
        return 0;
    }

    sort(num.begin(),num.end());

    int i=0,ans=0;
    while(kind>k){
        if(num[i]==0){
            i++;
            continue;
        }
        kind--;
        ans+=num[i];
        i++;
    }

    cout << ans << endl;

    return 0;
}
