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

    int a,b;
    cin >> a >> b;

    if(a==1 && b!=1){
        cout << "Alice" << endl;
        return 0;
    }else if(a!=1 && b==1){
        cout << "Bob" << endl;
        return 0;
    }else if(a==1 && b==1){
        cout << "Draw" << endl;
        return 0;
    }

    if(a>b){
        cout << "Alice" << endl;
    }else if(a<b){
        cout << "Bob" << endl;
    }else{
        cout << "Draw" << endl;
    }

    return 0;
}
