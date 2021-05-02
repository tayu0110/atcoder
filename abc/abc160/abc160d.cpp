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
    int n, x, y;
    cin >> n >> x >> y;

    vector<int> distlist(n-1);
    for(int i=1;i<=n;i++){
        for(int j=i+1;j<=n;j++){
            if(i<=x && j>=y){
                distlist.at(x-i+j-y)++;
            }else{
                distlist.at(j-i-1)++;
            }
        }
    }

    for(auto x:distlist)
        cout << x << endl;

    return 0;
}
