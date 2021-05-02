#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);
    int n;
    cin >> n;
    vector<ll> a(n);
    for(int i=0;i<n;i++){
        cin >> a[i];
    }
    sort(a.begin(),a.end());
    vector<ll> ans(0),rans(0);
    ll diff=0,rdiff=0;
    for(int i=0;i<n;i++){
        if(n-1-i<i)break;
        else if(n-1-i==i){
            if(!ans.empty())diff+=abs(ans[ans.size()-1]-a[i]);
            ans.push_back(a[i]);
            if(!rans.empty())rdiff+=abs(rans[rans.size()-1]-a[n-1-i]);
            rans.push_back(a[n-1-i]);
        }else{
            if(!ans.empty())diff+=abs(ans[ans.size()-1]-a[i]);
            ans.push_back(a[i]);
            ans.push_back(a[n-1-i]);
            diff+=abs(a[i]-a[n-1-i]);
            if(!rans.empty())rdiff+=abs(rans[rans.size()-1]-a[n-1-i]);
            rans.push_back(a[n-1-i]);
            rans.push_back(a[i]);
            rdiff+=abs(a[i]-a[n-1-i]);
        }
    }
    if(abs(ans[0]-ans[ans.size()-1])>abs(ans[ans.size()-1]-ans[ans.size()-2]))diff+=abs(ans[0]-ans[ans.size()-1])-abs(ans[ans.size()-1]-ans[ans.size()-2]);
    if(abs(rans[0]-rans[rans.size()-1])>abs(rans[rans.size()-1]-rans[rans.size()-2]))rdiff+=abs(rans[0]-rans[rans.size()-1])-abs(rans[rans.size()-1]-rans[rans.size()-2]);
    cout << max(diff, rdiff) << endl;
    // for(int i=0;i<ans.size();i++){
    //     cout << "i: " << i << " ans[i]: " << ans[i] << endl;
    // }
    return 0;
}
