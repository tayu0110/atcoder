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
    int n,m;
    cin >> n >> m;

    vector<int> ac(n,0);
    int acnum=0,pen=0;
    for(int i=0;i<m;i++){
        int p;
        string s;
        cin >> p >> s;

        if(ac[p-1]==-1){
            continue;
        }else if(ac[p-1]!=-1 && s=="WA"){
            ac[p-1]++;
        }else if(ac[p-1]!=-1 && s=="AC"){
            pen+=ac[p-1];
            ac[p-1]=-1;
            acnum++;
        }
    }

    cout << acnum << " " << pen << endl;

    return 0;
}
