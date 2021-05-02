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

int main(int argc,char* argv[]){
    int k, n;
    cin >> k >> n;

    vector<int> a(n);
    for(auto &x:a)
        cin >> x;
    
    sort(a.begin(), a.end());

    a.push_back(a.at(0)+k);

    vector<int> dist(n);
    int maxdist=-1;
    for(int i=0;i<n;i++){
        dist.at(i)=a.at(i+1)-a.at(i);
        maxdist=max(maxdist, dist.at(i));
    }

    cout << k-maxdist << endl;

    return 0;
}
