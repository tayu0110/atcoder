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
    int n,k;
    cin >> n >> k;

    vector<ll> a(n);
    for(auto &x:a)
        cin >> x;

    vector<bool> judge(n,false);
    for(int i=k;i<n;i++){
        if(a.at(i)>a.at(i-k)) judge.at(i)=true;
    }

    for(int i=k;i<n;i++){
        if(judge.at(i)) cout << "Yes" << endl;
        else cout << "No" << endl;
    }

    return 0;
}
