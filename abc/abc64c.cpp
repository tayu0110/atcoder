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
    vector<int> a(n);
    for(auto &x:a)cin >> x;

    bool gray=false,brown=false,green=false,sky=false,blue=false,yellow=false,orange=false,red=false;
    int ansmin=0,ansmax=0;
    for(int i=0;i<n;i++){
        switch(a[i]/400){
            case 0:if(!gray){ansmin++;ansmax++;gray=true;}break;
            case 1:if(!brown){ansmin++;ansmax++;brown=true;}break;
            case 2:if(!green){ansmin++;ansmax++;green=true;}break;
            case 3:if(!sky){ansmin++;ansmax++;sky=true;}break;
            case 4:if(!blue){ansmin++;ansmax++;blue=true;}break;
            case 5:if(!yellow){ansmin++;ansmax++;yellow=true;}break;
            case 6:if(!orange){ansmin++;ansmax++;orange=true;}break;
            case 7:if(!red){ansmin++;ansmax++;red=true;}break;
            default:ansmax++;break;
        }
    }

    if(ansmin==0)ansmin++;

    cout << ansmin << " " << ansmax << endl;

    return 0;
}
