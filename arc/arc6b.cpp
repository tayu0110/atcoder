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

struct Edge {
    int to;
    long long weight;
    Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n,l;
    cin >> n >> l;
    vector<string> t(l+2);
    for(int i=0;i<l+2;i++){
        getline(cin, t[i]);
        // cout << t[i] << endl;
    }
    int len=t[1].size();
    int h=0;
    for(int i=l+1;i>=1;i--){
        string s=t[i];
        // cout << "len: " << len << endl;
        // cout << "s: " << s << endl;
        if(i==l+1){
            for(int j=0;j<len;j++){
                if(s[j]=='o'){
                    h=j;
                    break;
                }
            }
        }else{
            if(h-1>=0 && s[h-1]=='-'){
                h-=2;
            }else if(h+1<len && s[h+1]=='-'){
                // cout << "reached" << endl;
                h+=2;
            }
        }
        // cout << h << endl;
    }
    cout << h/2+1 << endl;
    return 0;
}
