#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
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

    int h,w;
    cin >> h >> w;
    vector<string> plane(h);
    for(auto &x:plane)cin >> x;

    string sample(w,'.');
    for(int i=0;i<h;i++){
        if(sample==plane[i]){
            plane.erase(plane.begin()+i);
            i--;
            h--;
        }
    }

    set<int> jump;
    for(int i=0;i<w;i++){
        bool check=true;
        for(int j=0;j<h;j++){
            if(plane[j][i]=='#'){
                check=false;
                break;
            }
        }
        if(check)jump.insert(i);
    }

    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            if(jump.count(j))continue;
            cout << plane[i][j];
        }
        cout << endl;
    }

    return 0;
}
