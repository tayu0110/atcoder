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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    ll n,k,x,y;

    cin >> n >> k >> x >> y;

    if(n>k)
        cout << k*x+(n-k)*y << endl;
    else
        cout << n*x << endl;

    return 0;
}
