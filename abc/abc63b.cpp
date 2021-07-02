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
    string s;
    cin >> s;

    set<char> c;
    bool ans=true;
    for(int i=0;i<s.size();i++){
        if(c.count(s[i])){
            ans=false;
            break;
        }
        c.insert(s[i]);
    }

    if(ans)cout << "yes" << endl;
    else cout << "no" << endl;

    return 0;
}
