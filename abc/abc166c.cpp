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
    vector<ll> h(n);
    for(int i=0;i<n;i++){
        cin >> h[i];
    }
    vector<vector<int>> way(n,vector<int>(0));
    vector<int> a(m),b(m);
    for(int i=0;i<m;i++){
        cin >> a[i] >> b[i];
        a[i]--;b[i]--;
        way[a[i]].push_back(b[i]);
        way[b[i]].push_back(a[i]);
    }
    int count=0;
    // set<int> check;
    for(int i=0;i<n;i++){
        // if(check.find(i)!=check.end())continue;
        bool flag=true;
        for(int j=0;j<way.at(i).size();j++){
            int opp=way.at(i).at(j);
            if(h.at(i)<=h.at(opp)){
                flag=false;
            }
                // check.insert(i);
            // }else{
                // check.insert(opp);
            // }
        }
        if(flag)count++;
    }
    cout << count << endl;
    return 0;
}
