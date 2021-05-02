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

    vector<int> k(n);
    vector<int> ans(m+1);
    for(int i=0;i<n;i++){
        cin >> k[i];
        for(int j=0;j<k[i];j++){
            int aval;
            cin >> aval;
            ans[aval]++;
        }
    }

    int count=0;
    for(int i=1;i<m+1;i++){
        if(ans[i]==n)count++;
    }

    cout << count << endl;

    return 0;
}
