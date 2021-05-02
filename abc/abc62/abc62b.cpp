#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
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
    int h,w;
    cin >> h >> w;

    vector<vector<char>> pic(h+2,vector<char>(w+2,'#'));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cin >> pic[i+1][j+1];
        }
    }

    for(int i=0;i<h+2;i++){
        for(int j=0;j<w+2;j++){
            cout << pic[i][j];
        }
        cout << endl;
    }

    return 0;
}
