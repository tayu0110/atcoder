#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
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
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    string s;
    cin >> s;

    if(s.substr(0,7)=="keyence" || s.substr(s.size()-7,7)=="keyence"){
        cout << "YES" << endl;
        return 0;
    }

    bool ans=false;
    string str="keyence";
    for(int i=1;i<7;i++){
        if(s.substr(0,i)==str.substr(0,i) && s.substr(s.size()-7+i,7-i)==str.substr(i)){
            ans=true;
            break;
        }
    }

    if(ans)cout << "YES" << endl;
    else cout << "NO" << endl;

    return 0;
}
