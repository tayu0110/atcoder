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
    string s;
    ll k;
    cin >> s >> k;

    ll count=0;
    char ans=0;
    for(int i=0;i<s.size();i++){
        if(s[i]=='1'){
            count++;
            if(count==k){
                ans='1';
                break;
            }
        }else{
            ans=s[i];
            break;
        }
    }

    cout << ans << endl;

    return 0;
}
