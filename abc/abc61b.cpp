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
    int n,m;
    cin >> n >> m;

    vector<int> city(n+1);
    for(int i=0;i<m;i++){
        int a,b;
        cin >> a >> b;

        city[a]++;
        city[b]++;
    }

    for(int i=1;i<n+1;i++){
        cout << city[i] << endl;
    }

    return 0;
}