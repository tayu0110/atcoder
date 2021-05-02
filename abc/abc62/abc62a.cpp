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

    int x,y;
    cin >> x >> y;

    bool ans=false;
    set<int> group1={1,3,5,7,8,10,12},group2={4,6,9,11},group3={2};
    if(group1.find(x)!=group1.end() && group1.find(y)!=group1.end())ans=true;
    if(group2.find(x)!=group2.end() && group2.find(y)!=group2.end())ans=true;
    if(group3.find(x)!=group3.end() && group3.find(y)!=group3.end())ans=true;

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
