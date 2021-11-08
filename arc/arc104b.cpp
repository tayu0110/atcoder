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

    int n;
    cin >> n;
    string s;
    cin >> s;

    int atcg[4][5000];
    for(int i=0;i<4;i++){
        atcg[i][0]=0;
    }
    if(s[0]=='A')atcg[0][0]++;
    else if(s[0]=='T')atcg[1][0]++;
    else if(s[0]=='C')atcg[2][0]++;
    else if(s[0]=='G')atcg[3][0]++;
    for(int i=1;i<s.size();i++){
        for(int j=0;j<4;j++){
            atcg[j][i]=atcg[j][i-1];
        }
        if(s[i]=='A')atcg[0][i]++;
        else if(s[i]=='T')atcg[1][i]++;
        else if(s[i]=='C')atcg[2][i]++;
        else if(s[i]=='G')atcg[3][i]++;
    }

    ll ans=0;
    for(int i=0;i<n-1;i++){
        for(int j=i+1;j<n;j++){
            if((j-i)%2==0)continue;
            int a,t,c,g;
            if(i==0){
                a=atcg[0][j];
                t=atcg[1][j];
                c=atcg[2][j];
                g=atcg[3][j];
            }else{
                a=atcg[0][j]-atcg[0][i-1];
                t=atcg[1][j]-atcg[1][i-1];
                c=atcg[2][j]-atcg[2][i-1];
                g=atcg[3][j]-atcg[3][i-1];
            }
            // cout << a << t << c << g << endl;
            if(a==t && c==g){
                ans++;
            }
        }
        // cout << "i:"<<i << endl;
    }

    cout << ans << endl;

    return 0;
}
