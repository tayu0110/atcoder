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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    int h,w;
    cin >> h >> w;

    vector<vector<char>> pic(h,vector<char>(w));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cin >> pic[i][j];
        }
    }

    vector<vector<char>> ans(2*h,vector<char>(w));
    for(int i=0;i<2*h;i++){
        for(int j=0;j<w;j++){
            ans.at(i).at(j)=pic.at(i/2).at(j);
        }
    }

    for(int i=0;i<2*h;i++){
        for(int j=0;j<w;j++){
            cout << ans[i][j];
        }
        cout << endl;
    }

    return 0;
}
