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
    double d;
    cin >> n >> d;

    vector<pair<long double,long double>> xy(n);

    for(auto &x:xy)
        cin >> x.first >> x.second;

    int count=0;
    for(int i=0;i<n;i++){
        long double distance;
        long double x=xy.at(i).first,y=xy.at(i).second;
        distance=sqrtl(x*x+y*y);
        if(distance<=d)count++;
    }

    cout << count << endl;

    return 0;
}
