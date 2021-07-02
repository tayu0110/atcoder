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
    ll n;
    cin >> n;



    string nstr=to_string(n);
    bool check=true;
    for(int i=1;i<nstr.size();i++){
        if(nstr[i]!='9'){
            check=false;
            break;
        }
    }
    if(check){
        int top=nstr[0]-'0';
        cout << 9*(nstr.size()-1)+top << endl;
    }else{
        int top=(nstr[0]-'0')-1;
        int ans=9*(nstr.size()-1)+top;
        cout << ans << endl;
    }

    return 0;
}
