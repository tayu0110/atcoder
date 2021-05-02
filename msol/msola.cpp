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
#define INF (1LL<<62)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    if(n<=599) cout << 8 << endl;
    else if(n>=600 && n<=799) cout << 7 << endl;
    else if(n>=800 && n<=999) cout << 6 << endl;
    else if(n>=1000 && n<=1199) cout << 5 << endl;
    else if(n>=1200 && n<=1399) cout << 4 << endl;
    else if(n>=1400 && n<=1599) cout << 3 << endl;
    else if(n>=1600 && n<=1799) cout << 2 << endl;
    else if(n>=1800) cout << 1 << endl;

    return 0;
}
