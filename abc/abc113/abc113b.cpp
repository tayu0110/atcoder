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
    double n,a,t;
    cin >> n >> t >> a;

    vector<double> h(n);
    for(auto &x:h)cin >> x;

    double best=inf,bestpt=-1;
    for(int i=0;i<n;i++){
        double temp=t-h[i]*0.006;
        if(best>abs(a-temp)){
            best=abs(a-temp);
            bestpt=i;
        }
    }

    cout << bestpt+1 << endl;

    return 0;
}
