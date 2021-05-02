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

    string s,t;
    cin >> s >> t;

    sort(s.begin(),s.end());
    sort(t.rbegin(),t.rend());

    bool ans=false;
    for(int i=0;i<s.size();i++){
        if(s[i]<t[i]){
            ans=true;
            break;
        }else if(s[i]>t[i]){
            break;
        }else{
            if(s.size()<t.size()){
                if(i==s.size()-1){
                    ans=true;
                    break;
                }
            }else{
                if(i==t.size()-1){
                    break;
                }
            }
        }
    }

    if(ans) cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
