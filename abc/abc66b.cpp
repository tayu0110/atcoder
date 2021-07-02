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

    int n=s.size()-2,ans=0;
    for(int i=n-1;i>=0;i-=2){
        bool check=true;
        for(int j=i;j>=(i+1)/2;j--){
            if(s[j]!=s[j-(i+1)/2]){
                check=false;
                break;
            }
        }
        if(check){
            ans=i+1;
            break;
        }
    }

    cout << ans << endl;

    return 0;
}
