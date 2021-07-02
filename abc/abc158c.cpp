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
    int a,b;
    cin >> a >> b;

    int ans=-1;
    for(int i=1;i<=1000;i++){
        int tax8=i*108/100-i;
        int tax10=i*110/100-i;

        if(tax8==a && tax10==b){
            ans=i;
            break;
        }
    }

    cout << ans << endl;

    return 0;
}
