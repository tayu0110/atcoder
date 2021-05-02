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
    int n;
    cin >> n;
    vector<string> w(n);
    for(int i = 0; i < n; i++) {
        cin >> w[i];
    }
    vector<string> res(0);
    for(int i = 0; i < n; i++) {
        string s = w[i];
        string ans;
        for(int j = 0; j < s.length(); j++) {
            char c = s[j];
            c = tolower(c);
            if(c == 'b' || c == 'c') ans += "1";
            else if(c == 't' || c == 'j') ans += "3";
            else if(c == 'l' || c == 'v') ans += "5";
            else if(c == 'p' || c == 'm') ans += "7";
            else if(c == 'n' || c == 'g') ans += "9";
            else if(c == 'd' || c == 'w') ans += "2";
            else if(c == 'f' || c == 'q') ans += "4";
            else if(c == 's' || c == 'x') ans += "6";
            else if(c == 'h' || c == 'k') ans += "8";
            else if(c == 'z' || c == 'r') ans += "0";
            else continue;
        }
        if(ans != "") res.push_back(ans);
    }
    for(int i = 0; i < res.size(); i++) {
        string s = res[i];
        cout << s;
        if(i != res.size()-1) cout << " ";
    }
    cout << endl;
    return 0;
}
