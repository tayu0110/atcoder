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
    for(int i=0;i<n;i++){
        vector<int> f(n,-1);
        f[i]=0;
        queue<int> nt;
        nt.push(i);
        int count=0;
        while(!nt.empty()) {
            int now=nt.front();
            nt.pop();
            for(int j=0;j<t[now].size();j++){
                int k=t[now][j];
                if(f[k]!=-1)continue;
                nt.push(k);
                f[k]=f[now]+1;
                if(f[k]==2) count++;
            }
        }
        cout << count << endl;
    }
    return 0;
}
