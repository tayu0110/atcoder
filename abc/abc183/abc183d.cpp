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
    ll w;
    cin >> w;
    vector<pair<pii,ll>> stp(n);
    for(int i=0;i<n;i++){
        int s,t;
        ll p;
        cin >> s >> t >> p;
        stp[i]=make_pair(make_pair(s,t),p);
    }
    sort(stp.begin(),stp.end());
    queue<pair<pii,ll>> li;
    for(int i=0;i<n;i++){
        int s,t;
        ll p;
        s=stp[i].first.first;
        t=stp[i].first.second;
        p=stp[i].second;
        li.push(make_pair(make_pair(s,t),p));
    }
    map<int,ll> fin;
    ll lost=0;
    for(int i=0;i<200010;i++){
        while(!li.empty()){
            pair<pii,ll> fr=li.front();
            int s=fr.first.first,t=fr.first.second;
            ll p=fr.second;
            if(s!=i)break;
            if(fin.find(t)!=fin.end()){
                fin[t]+=p;
            }else{
                fin.insert(make_pair(t,p));
            }
            lost+=fr.second;
            li.pop();
        }
        while(!fin.empty()){
            if(fin.find(i)==fin.end())break;
            lost-=fin[i];
            fin.erase(i);
        }
        if(w-lost<0){
            cout << "No" << endl;
            return 0;
        }
        if(li.empty() && fin.empty())break;
    }

    cout << "Yes" << endl;

    return 0;
}
