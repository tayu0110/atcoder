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
    cin >> n;

    vector<ll> l(n);
    for(auto &x:l)
        cin >> x;
    
    sort(l.begin(),l.end());
    
    ll ans=0;
    for(int i=0;i<n;i++){
        for(int j=i+1;j<n;j++){
            if(l.at(i)==l.at(j))
                continue;
            for(int k=j+1;k<n;k++){
                if(l.at(j)==l.at(k))
                    continue;
                if(l.at(i)+l.at(j)>l.at(k))
                    ans++;
            }
        }
    }

    cout << ans << endl;

    return 0;
}
