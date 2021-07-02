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
    int n,k,l;
    cin >> n >> k >> l;

    vector<pii> road(k),rail(l);
    for(auto &x:road)
        cin >> x.first >> x.second;
    for(auto &x:rail)
        cin >> x.first >> x.second;
    
    

    return 0;
}
