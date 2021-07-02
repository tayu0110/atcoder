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
    int n,k;
    cin >> n >> k;

    vector<int> p(n);
    for(auto &x:p)
        cin >> x;
    
    sort(p.begin(),p.end());

    ll sum=0;
    for(int i=0;i<k;i++){
        sum+=p.at(i);
    }

    cout << sum << endl;

    return 0;
}
