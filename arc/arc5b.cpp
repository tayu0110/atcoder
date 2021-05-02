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
    int x, y;
    string w;
    cin >> x >> y >> w;
    // cout << "w: " << w << endl;
    vector<string> c(9);
    for(int i=0;i<9;i++) cin >> c[i];
    x--;y--;
    pii l[4];
    l[0].first=x;l[0].second=y;
    for(int i=1;i<4;i++){
        int px=l[i-1].first, py=l[i-1].second;
        if(w=="R"){
            px++;
            // cout << "reached R" << endl;
        }else if(w=="L"){
            px--;
        }else if(w=="U"){
            py--;
        }else if(w=="D"){
            py++;
        }else if(w=="RU"){
            px++;
            py--;
        }else if(w=="RD"){
            px++;
            py++;
        }else if(w=="LU"){
            px--;
            py--;
        }else if(w=="LD"){
            px--;
            py++;
        }
        // cout << "px: " << px << " py: " << py << endl;
        l[i].first=px;l[i].second=py;
    }
    for(int i=0;i<4;i++){
        int px=l[i].first, py=l[i].second;
        if(px < 0) px=-px;
        else if(px > 8) px=8-(px-8);
        if(py < 0) py=-py;
        else if(py > 8) py=8-(py-8);
        // cout << "px: " << px << " py: " << py << endl;
        cout << c[py][px];
        // cout << endl;
    }
    cout << endl;
    return 0;
}
