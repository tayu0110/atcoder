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
    int n,k;
    cin >> n >> k;
    string s;
    cin >> s;
    vector<int> i(1,0);
    i.push_back(1);
    for(int j=1;j<s.size();j++){
        if(s[j]!=s[j-1]) i.push_back(j+1);
    }
    for(int j=0;j<i.size();j++){
        // cout << "j: " << j << " i[j]: " << i[j] << endl;
    }
    int ans=1;
    for(int K=1;K<i.size();K++){
        int ik=i[K]-1;
        if(s[ik]=='0'){
            if(K+2*k<i.size())ans=max(ans, i[K+2*k]-i[K]);
            else ans=max(ans, n+1-i[K]);
        }
        else{
            if(K+2*k+1<i.size())ans=max(ans, i[K+2*k+1]-i[K]);
            else ans=max(ans,n+1-i[K]);
        }
        // cout << "K: " << K << " ans: " << ans << endl;
    }
    cout << ans << endl;
    // if(n==1){
    //     cout << 1 << endl;
    //     return 0;
    // }
    // vector<int> b(1,0),w(1,0);
    // bool bflag=true,wflag=false;
    // int now=0,count=0;
    // while(now<n) {
    //     if(bflag){
    //         if(s[now]=='1'){
    //             count++;
    //             now++;
    //         } else {
    //             b.push_back(count);
    //             bflag=false;
    //             wflag=true;
    //             count=0;
    //         }
    //     } else if(wflag) {
    //         if(s[now]=='0'){
    //             count++;
    //             now++;
    //         }else {
    //             w.push_back(count);
    //             wflag=false;
    //             bflag=true;
    //             count=0;
    //         }
    //     }
    // }
    // if(bflag)b.push_back(count);
    // else if(wflag) w.push_back(count);
    // if(b.size()==w.size())b.push_back(0);
    // else if(b.size()<w.size()) while(b.size()!=w.size()+1)b.push_back(0);
    // else if(b.size()-1>w.size()) while(b.size()!=w.size()+1)w.push_back(0);
    // for(int i=1;i<b.size();i++){
    //     b[i]+=b[i-1];
    //     // cout << "b[i]: " << b[i] << endl;
    // }
    // for(int i=1;i<w.size();i++){
    //     w[i]+=w[i-1];
    //     // cout << "w[i]: " << w[i] << endl;
    // }
    // int ans=0;
    // for(int i=1;i+k<b.size();i++){
    //     int back=0;
    //     // cout << b[i+k] << " " << b[i-1] << " " << w[i+k-1] << " " << w[i] << endl;
    //     back=b[i+k]-b[i-1]+w[i+k-1]-w[i-1];
    //     ans=max(ans, back);
    //     // cout << "i: " << i << " ans: " << ans << endl;
    // }
    // cout << ans << endl;
    return 0;
}
