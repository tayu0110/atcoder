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

    if(s[0]==s[1] && s[1]==s[2] && s[0]=='R')
        cout << 3 << endl;
    else if(s[0]==s[1] && s[0]=='R')
        cout << 2 << endl;
    else if(s[2]==s[1] && s[1]=='R')
        cout << 2 << endl;
    else if(s[0]=='R' || s[1]=='R' || s[2]=='R')
        cout << 1 << endl;
    else
        cout << 0 << endl;

    return 0;
}
