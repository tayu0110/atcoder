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
    vector<pair<pair<ll, int>,int>> yp(m);
    for(int i=0;i<m;i++){
        int p;
        ll y;
        cin >> p >> y;
        yp[i]=make_pair(make_pair(y, p),i);
    }
    sort(yp.begin(), yp.end());
    vector<vector<ll>> city(n+1, vector<ll>(0));
    map<int, string> ans;
    for(int i=0;i<m;i++){
        ll y=yp[i].first.first;
        int p=yp[i].first.second;
        int j=yp[i].second;
        int x=city[p].size()+1;
        city[p].push_back(y);
        string post="000000"+to_string(p);
        string nx="000000"+to_string(x);
        string res=post.substr(post.size()-6,6)+nx.substr(nx.size()-6,6);
        ans.insert(make_pair(j,res));
    }
    for(auto it=ans.begin();it!=ans.end();it++){
        cout << it->second << endl;
    }
    return 0;
}
