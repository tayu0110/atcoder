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
    int A,B,M;
    cin >> A >> B >> M;

    vector<int> a(A),b(B);
    int amin=inf,bmin=inf;
    for(auto &x:a){
        cin >> x;
        amin=min(amin,x);
    }
    for(auto &x:b){
        cin >> x;
        bmin=min(bmin,x);
    }

    int minval=amin+bmin;

    for(int i=0;i<M;i++){
        int x,y,c;
        cin >> x >> y >> c;

        minval=min(minval,(a[x-1]+b[y-1]-c));
    }

    cout << minval << endl;

    return 0;
}
