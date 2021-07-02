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
    int n,a,b;
    cin >> n >> a >> b;

    int ans=0;
    for(int i=1;i<=n;i++){
        string s=to_string(i);
        int sum=0;
        for(int j=0;j<s.size();j++){
            sum+=(s[j]-'0');
        }
        if(sum>=a && sum<=b)ans+=i;
    }

    cout << ans << endl;

    return 0;
}
