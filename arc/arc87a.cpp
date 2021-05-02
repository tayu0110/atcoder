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

    map<ll, int> check;
    for(int i=0;i<n;i++){
        ll a;
        cin >> a;

        if(check.find(a)!=check.end()){
            check[a]++;
        }else{
            check.insert(make_pair(a, 1));
        }
    }

    int ans=0;
    for(auto it=check.begin();it!=check.end();it++){
        ll x=it->first;
        int count=it->second;
        if(x==count)continue;
        if(x>count){
            ans+=count;
        }else{
            ans+=count-x;
        }
    }

    cout << ans << endl;

    return 0;
}
