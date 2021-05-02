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
    int n;
    ll k;
    cin >> n >> k;

    priority_queue<long double> a;
    for(int i=0;i<n;i++){
        long double d;
        cin >> d;
        a.push(d);
    }

    for(int i=0;i<k;i++){
        long double d,e;
        d=e=a.top()/2;
        a.pop();
        a.push(d);
        a.push(e);
    }

    cout << a.top() << endl;

    return 0;
}
