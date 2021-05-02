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
    string s;
    cin >> s;
    int k;
    cin >> k;
    set<string> ck;
    int len = s.size();
    for(int i=0;i<len;i++){
        string now="";
        for(int j=i;j<min(len,i+k);j++){
            now+=s[j];
            ck.insert(now);
            // cout << "i: " << i << " j: " << j << " s: " << now << endl;
        }
    }
    auto it=ck.begin();
    string ans="";
    for(int i=0;i<k;i++){
        ans=*it;
        it++;
    }
    cout << ans << endl;
    return 0;
}
