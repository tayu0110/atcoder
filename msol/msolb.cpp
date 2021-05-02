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
    int a,b,c;
    cin >> a >> b >> c;
    int n;
    cin >> n;

    for(int i=0;i<n;i++){
        if(a>=b) b*=2;
        else if(a<b && b>=c) c*=2;
        else if(a<b && b<c) break;
    }

    if(a<b && b<c){
        cout << "Yes" << endl;
    }else{
        cout << "No" << endl;
    }

    return 0;
}
