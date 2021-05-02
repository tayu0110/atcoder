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

    int n;
    cin >> n;
    string s,t;
    cin >> s >> t;

    int ans=0;
    for(int i=0;i<n;i++){
        int count=0;
        for(int j=0;i+j<n;j++){
            if(s[i+j]==t[j])count++;
            else{
                count=0;
                break;
            }
        }
        if(count!=0){
            ans=count;
            break;
        }
    }

    cout << s.size()+t.size()-ans << endl;

    return 0;
}
