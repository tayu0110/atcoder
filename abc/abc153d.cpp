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
    ll h;
    cin >> h;

    ll count=0;
    ll plus=1;
    while(h!=0){
        h/=2;
        count+=plus;
        plus*=2;
    }

    // queue<ll> monster;
    // monster.push(h);
    // ll count = 0;
    // while(!monster.empty()){
    //     if(monster.front()==1){
    //         monster.pop();
    //         count++;
    //         continue;
    //     }
    //     monster.push(monster.front()/2);
    //     monster.push(monster.front()/2);
    //     monster.pop();
    //     count++;
    // }

    cout << count << endl;

    return 0;
}
