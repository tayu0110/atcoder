//黒く塗る部分に注意し、計算を効率化。
//配列の要素の数え上げの際、最後の要素or最初の要素の扱いに注意
//→データ構造からして、要素の数え上げの関数のあるものを使う。
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
using pll = pair<ll,ll>;

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    ll h,w,n;
    cin >> h >> w >> n;

    if(n==0){
        cout << (h-2)*(w-2) << endl;
        for(int i=1;i<10;i++)
            cout << "0" << endl;
        return 0;
    }

    vector<pll> ab(n);
    for(auto &x:ab){
        ll a,b;
        cin >> a >> b;
        x=make_pair(a,b);
    }

    vector<pll> sqare(0);
    for(auto x:ab){
        for(int i = 0; i < 3; i++){
            for(int j = 0; j < 3; j++){
                sqare.push_back(make_pair(x.first-i,x.second-j));
            }
        }
    }

    sort(sqare.begin(),sqare.end());

    int count=1;
    vector<ll> ans(10);
    ans[0]=(h-2)*(w-2);
    for(ll i=0;i<sqare.size();i++){
        if(sqare[i].first<=0 || sqare[i].second<=0 || sqare[i].first+2>h || sqare[i].second+2>w)
            continue;
        if(i==sqare.size()-1){
          if(sqare[i]!=sqare[i-1]){
            ans[1]++;
            ans[0]--;
            break;
          }else{
            break;
          }
        }
        if(sqare[i]==sqare[i+1]){
            count++;
        }else{
            ans[count]++;
            count=1;
            ans[0]--;
        }
    }

    for(auto x:ans)
        cout << x << endl;

    return 0;
}
