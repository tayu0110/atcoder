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

    vector<int> p(n),q(n);
    for(auto &x:p)cin >> x;
    for(auto &x:q)cin >> x;

    vector<int> perm(n,0);
    for(int i=0;i<n;i++){
        perm[i]=i;
    }

    int a=-1,b=-1;
    int count=1;
    do{
        if(a!=-1 && b!=-1)break;

        for(int i=0;i<n;i++){
            if(a!=-1)break;
            if(p[i]!=perm[i]+1)break;
            if(i==n-1)a=count;
        }
        for(int i=0;i<n;i++){
            if(b!=-1)break;
            if(q[i]!=perm[i]+1)break;
            if(i==n-1)b=count;
        }

        count++;
    }while(next_permutation(perm.begin(),perm.end()));

    cout << abs(a-b) << endl;

    return 0;
}
