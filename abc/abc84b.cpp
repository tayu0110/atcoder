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
    string s;
    cin >> a >> b >> s;

    bool ans=true;
    for(int i=0;i<s.size();i++){
        if(i==a){
            if(s[i]!='-'){
                ans = false;
                break;
            }
        }else{
            if(s[i]-'0'<0 || s[i]-'0'>9){
                ans = false;
                break;
            }
        }
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
