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
#define INF (1LL<<62)           //1LL<<63でオーバーフロー

int main(int argc,char* argv[]){
    int h,w,k;
    cin >> h >> w >> k;

    int scount=0;
    vector<vector<char>> field(h,vector<char>(w));
    for(int i=0;i<h;i++){
        for(int j=0;j<w;j++){
            cin >> field[i][j];
            if(field[i][j]=='#')
                scount++;
        }
    }

    if(k==scount){
        cout << "1" << endl;
        return 0;
    }else if(k>scount){
        cout << "0" << endl;
        return 0;
    }

    int ans=0;
    for(int i=0;i<(1<<h);i++){
        vector<vector<char>> cf(h,vector<char>(w));
        cf=field;
        for(int j=0;j<h;j++){
            if((1<<j) & i){
                for(int l=0;l<w;l++){
                    if(field[j][i]=='#')

                    field[j][l]='-';
                }
            }
        }

    }

    return 0;
}
