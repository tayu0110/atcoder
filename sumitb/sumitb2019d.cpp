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
    int n;
    cin >> n;
    string s;
    cin >> s;
    vector<vector<int>> ck(10, vector<int>(0));
    for(int i=0;i<s.size();i++){
        int k=s[i]-'0';
        ck[k].push_back(i);
    }
    int p,c,b;
    int ans=0;
    for(int i=0;i<10;i++){
        if(ck[i].size()==0)continue;
        p=ck[i][0];
        for(int j=0;j<10;j++){
            bool flag=false;
            for(int l=0;l<ck[j].size();l++){
                if(ck[j][l]>p){
                    c=ck[j][l];
                    flag=true;
                    break;
                }
            }
            if(!flag)continue;
            for(int k=0;k<10;k++){
                for(int m=0;m<ck[k].size();m++){
                    if(ck[k][m]>c){
                        ans++;
                        break;
                    }
                }
            }
        }
    }
    cout << ans << endl;
    return 0;
}
