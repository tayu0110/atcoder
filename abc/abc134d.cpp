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
    vector<int> a(n+1);
    for(int i=1;i<n+1;i++){
        cin >> a[i];
    }
    vector<int> box(n+1,0);
    queue<int> ans;
    for(int i=n;i>=0;i--){
        if(box[i]%2==a[i])continue;
        ans.push(i);
        for(int j=1;j*j<=i;j++){
            if(i%j!=0)continue;
            box[j]++;
            if(i/j!=j)box[i/j]++;
        }
    }
    cout << ans.size() << endl;
    while(!ans.empty()){
        cout << ans.front() << " ";
        ans.pop();
    }
    return 0;
}
