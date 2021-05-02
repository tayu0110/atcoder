#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using ld = long double;
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
    vector<int> b(n);
    for(int i=0;i<n;i++){
        cin >> b[i];
        b[i]--;
    }
    vector<int> ans(0);
    while(!b.empty()){
        bool flag=false;
        for(int i=b.size()-1;i>=0;i--){
            if(b[i]==i){
                ans.push_back(i);
                flag=true;
                b.erase(b.begin()+i);
                break;
            }
        }
        if(!flag){
            cout << -1 << endl;
            return 0;
        }
    }
    for(int i=ans.size()-1;i>=0;i--){
        cout << ans[i]+1 << endl;
    }
    return 0;
}
