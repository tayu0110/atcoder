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
    int n;
    cin >> n;

    vector<int> f(n,0);

    int x,y,z;
    for(x=1;x<100;x++){
        for(y=1;y<100;y++){
            for(z=1;z<100;z++){
                if(((x+y)*(x+y)+(y+z)*(y+z)+(z+x)*(z+x))%2!=0)continue;
                int fans;
                fans=((x+y)*(x+y)+(y+z)*(y+z)+(z+x)*(z+x))/2;
                if(fans>n)continue;
                f.at(fans-1)++;
            }
        }
    }

    for(int i=0;i<n;i++){
        cout << f.at(i) << endl;
    }

    return 0;
}
