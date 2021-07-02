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
    int s;
    cin >> s;

    if(s==1 || s==2){
        cout << 4 << endl;
        return 0;
    }

    int ans=1;
    while(s!=4){
        if(s%2)s=s*3+1;
        else s/=2;
        ans++;
    }

    cout << ans + 3 << endl;

    return 0;
}
