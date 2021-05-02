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
    vector<ll> a(n);
    for(auto &x:a)cin >> x;

    deque<ll> ans;
    for(int i=0;i<n;i++){
        if(i%2)ans.push_back(a[i]);
        else ans.push_front(a[i]);
    }

    for(int i=0;i<n;i++){
        if(n%2){
            cout << ans.front() << " ";
            ans.pop_front();
        }else{
            cout << ans.back() << " ";
            ans.pop_back();
        }
    }

    return 0;
}
