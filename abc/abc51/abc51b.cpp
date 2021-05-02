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
    int k, s;
    cin >> k >> s;

    ll count=0;
    for(int x=0;x<=k;x++){
        for(int y=0;y<=k;y++){
            int z=s-x-y;
            if(z>=0 && z<=k) count++;
        }
    }

    cout << count << endl;

    return 0;
}
