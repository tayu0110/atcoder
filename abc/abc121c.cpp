#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
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
    int n,m;
    cin >> n >> m;

    vector<pair<ll,int>> ab(n);
    for(auto &x:ab)cin >> x.first >> x.second;
    sort(ab.begin(),ab.end());

    ll ans=0;
    int buy=m,count=0;
    while(buy!=0){
        if(buy>=ab[count].second){
            ans+=ab[count].first*ab[count].second;
            buy-=ab[count].second;
        }else{
            ans+=ab[count].first*buy;
            buy=0;
        }
        count++;
    }

    cout << ans << endl;

    return 0;
}
