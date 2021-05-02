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
    int x;
    cin >> x;

    if(x<100){
        cout << 0 << endl;
        return 0;
    }

    int y=x%100;
    int pc=y/5;
    y=y%5;
    int ame=y/4;
    y%=4;
    int cake=y/3;
    y%=3;
    int cookie=y/2;
    y%=2;
    int sand=y;

    if(pc+ame+cake+cookie+sand<=x/100){
        cout << 1 << endl;
    }else{
        cout << 0 << endl;
    }

    return 0;
}
