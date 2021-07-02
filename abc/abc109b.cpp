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
    int n;
    cin >> n;

    vector<string> w(n);
    for(auto &x:w)cin >> x;

    set<string> check;
    bool ans = true;
    for(int i=0;i<n;i++){
        if(check.count(w[i])){
            ans = false;
            break;
        }
        check.insert(w[i]);
        if(i==0)continue;
        if(w[i][0]!=w[i-1].at(w[i-1].size()-1)){
            ans = false;
            break;
        }
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
