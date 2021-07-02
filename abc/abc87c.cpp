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
    int n;
    cin >> n;

    vector<int> up(n,0),down(n,0);
    vector<vector<int>> a(2,vector<int>(n,0));
    for(int i=0;i<2;i++){
        for(int j=0;j<n;j++){
            cin >> a[i][j];
            if(j>0){
                if(i==0)up[j]=a[i][j]+up[j-1];
                else down[j]=a[i][j]+down[j-1];
            }else{
                if(i==0)up[j]=a[i][j];
                else down[j]=a[i][j];
            }
        }
    }

    int maxval=up[0]+down[n-1]; 
    for(int i=1;i<n;i++){
        maxval=max(maxval, up[i]+down[n-1]-down[i-1]);
    }

    cout << maxval << endl;

    return 0;
}
