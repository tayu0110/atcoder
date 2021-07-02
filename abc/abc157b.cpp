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
    vector<vector<int>> bingo(3,vector<int>(3));
    for(int i=0;i<3;i++){
        for(int j=0;j<3;j++)
            cin >> bingo[i][j];
    }

    int n;
    cin >> n;
    
    vector<int> b(n);
    for(auto &x:b)cin >> x;

    for(int i=0;i<n;i++){
        for(int j=0;j<3;j++){
            for(int k=0;k<3;k++){
                if(b[i]==bingo[j][k]){
                    bingo[j][k]=-1;
                    break;
                }
            }
        }
    }

    int ans=0;
    for(int i=0;i<3;i++){
        if(bingo[i][0]==bingo[i][1] && bingo[i][1]==bingo[i][2])ans++;
        if(bingo[0][i]==bingo[1][i] && bingo[1][i]==bingo[2][i])ans++;        
    }
    if(bingo[0][0]==bingo[1][1] && bingo[1][1]==bingo[2][2])ans++;
    if(bingo[0][2]==bingo[1][1] && bingo[1][1]==bingo[2][0])ans++;

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
