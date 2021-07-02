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
    int n,d;
    cin >> n >> d;

    vector<vector<int>> x(n,vector<int>(d));
    for(int i=0;i<n;i++){
        for(int j=0;j<d;j++){
            cin >> x[i][j];
        }
    }

    int count=0;
    for(int i=0;i<n;i++){
        for(int j=i+1;j<n;j++){
            int distsqr=0;
            auto y=x[i];
            auto z=x[j];
            for(int k=0;k<d;k++){
                distsqr+=(y[k]-z[k])*(y[k]-z[k]);
            }

            for(int l=1;l*l<=distsqr;l++){
                if(l*l==distsqr)count++;
            }
        }
    }

    cout << count << endl;

    return 0;
}
