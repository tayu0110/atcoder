#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<numeric>
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
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int n;
    cin >> n;
    vector<int> a(n);
    map<int,int> list;
    for(auto &x:a){
        cin >> x;
        if(list.find(x)!=list.end())list.at(x)++;
        else list.insert(make_pair(x,1));
    }

    int maxval=0;
    for(int i=0;i<=100000;i++){
        int prev,nxt,now;
        if(list.find(i)==list.end())continue;
        else now=list.at(i);
        if(list.find(i-1)==list.end()){
            prev=0;
        }else{
            prev=list.at(i-1);
        }
        if(list.find(i+1)==list.end()){
            nxt=0;
        }else{
            nxt=list.at(i+1);
        }
        maxval=max(maxval,now+prev+nxt);
    }
    // for(auto it=list.begin();it!=list.end();it++){
    //     if(it==list.begin())continue;
    //     auto preit=it--,nxtit=it++;
    //     if(nxtit==list.end())break;
    //     if(it->first-preit->first!=1){
    //         if(nxtit->first-it->first!=1){
    //             maxval=max(maxval,it->second);
    //             continue;
    //         }else{
    //             maxval=max(maxval,it->second+nxtit->second);
    //             continue;
    //         }
    //     }else{
    //         if(nxtit->first-it->first!=1){
    //             maxval=max(maxval,it->second+preit->second);
    //             continue;
    //         }else{
    //             maxval=max(maxval,it->second+nxtit->second+preit->second);
    //             continue;
    //         }
    //     }
    // }

    cout << maxval << endl;

    return 0;
}
