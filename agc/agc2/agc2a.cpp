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
    ll a,b;
    cin >> a >> b;

    if(a<=0 && b>=0){
        cout << "Zero" << endl;
    }else if(a>0){
        cout << "Positive" << endl;
    }else if(b<0){
        if((abs(b-a)+1)%2==0)cout << "Positive" << endl;
        else cout << "Negative" << endl;
    }

    return 0;
}
