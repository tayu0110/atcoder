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
    int x,n;
    cin >> x >> n;
    
    vector<bool> checked(102,false);
    vector<int> p(n);
    for(auto i:p){
        cin >> i;
        checked.at(i)=true;
    }
    
    pii ans(999,-1);
    for(int i = 0; i < 102; i++){
        pii a(abs(x-i),i);
        if(checked.at(i)){
            continue;
        }
        ans=min(a,ans);
    }

    cout << ans.second << endl;

    return 0;
}
