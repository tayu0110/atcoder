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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    int n,m;
    cin >> n;

    vector<int> plb(n);
    for(auto &x:plb)
        cin >> x;
    
    cin >> m;
    for(int i=0;i<m;i++){
        int p,x,sum=0;
        cin >> p >> x;
        for(int j=0;j<n;j++){
            if(j+1==p)
                sum+=x;
            else
                sum+=plb[j];
        }
        cout << sum << endl;
    }

    return 0;
}
