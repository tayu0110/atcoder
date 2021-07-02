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

template<class T>T GCD(T a, T b){if(a<b){T temp=a;a=b;b=temp;}T res=a%b;while(res!=0){a=b;b=res;res=a%b;}return b;}

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    cin >> n;
    vector<ll> a(n);
    queue<ll> enemy;
    ll minene=INF;
    for(int i=0;i<n;i++){
        cin >> a[i];
        enemy.push(a[i]);
        minene=min(minene,a[i]);
    }

    while(!enemy.empty()){
        minene=GCD(enemy.front(),minene);
        enemy.pop();
    }
    cout << minene << endl;

    return 0;
}
