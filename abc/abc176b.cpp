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

int main(int argc,char* argv[]){
    string s;
    cin >> s;

    ll sum=0;
    for(int i=0;i<s.size();i++){
        sum += s[i]-'0';
    }

    if(sum%9==0) cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
