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
    int n,m;
    cin >> n >> m;
    vector<vector<int>> t(n,vector<int>(0));
    for(int i=0;i<m;i++){
        int a,b;
        cin >> a >> b;
        a--;b--;
        t[a].push_back(b);
        t[b].push_back(a);
    }
    vector<int> ans(n);
    queue<int> nt;
    nt.push(0);
    set<int> ck;
    ck.insert(0);
    int count=1;
    while(!nt.empty()) {
        int now=nt.front();
        nt.pop();
        for(int i=0;i<t[now].size();i++){
            int j=t[now][i];
            if(ck.find(j)!=ck.end()) continue;
            ck.insert(j);
            nt.push(j);
            ans[j]=now+1;
            count++;
            // cout << "now: " << now << " j: " << j << endl;
        }
    }
    if(count!=n) {
        cout << "No" << endl;
        return 0;
    } else {
        cout << "Yes" << endl;
        for(int i=1;i<ans.size();i++){
            cout << ans[i] << endl;
        }
    }
    return 0;
}
