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
    string q;
    getline(cin, q);
    q += ' ';
    vector<string> s(0);
    int p=0;
    for(int i=0;i<q.size();i++){
        if(q[i] == ' '){
            s.push_back(q.substr(p, i-p));
            p=i+1;
        }
    }
    int n;
    cin >> n;
    set<string> t;
    for(int i=0;i<n;i++){
        string c;
        cin >> c;
        t.insert(c);
    }
    for(int i=0;i<s.size();i++){
        string r=s[i];
        bool oflag=true;
        for(auto it=t.begin();it!=t.end();it++){
            string u=*it;
            int iflag=false;
            if(r.length() != u.length()) continue;
            for(int j=0;j<r.length();j++){
                if(r[j]==u[j] || u[j]=='*') continue;
                iflag=true;
                break;
            }
            if(!iflag) {
                for(int j=0;j<r.length();j++){
                    s[i][j]='*';
                }
                break;
            }
        }
    }
    for(int i=0;i<s.size();i++){
        cout << s[i];
        if(i != s.size()-1) cout << " ";
    }
    cout << endl;
    return 0;
}
