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
    vector<int> p(n);
    vector<int> ud(n);
    queue<pii> tr;
    for(int i=0;i<n;i++){
        cin >> p[i];
        p[i]--;
        if(p[i]==i){
            cout << -1 << endl;
            return 0;
        }else if(p[i]<i){
            ud[i]=-1;
        }else{
            ud[i]=1;
        }
    }

    // for(int i=0;i<n;i++){
    //     cout << "i: " << i << "ud[i]:" << ud[i] << endl;
    // }
    for(int i=0;i<n-1;i++){
        if(ud[i]==1 && ud[i+1]==-1){
            tr.push(make_pair(i,i+1));
            // cout << "i: " << i << " i+1: " << i+1 << endl;
        }
    }
    queue<int> ans;
    set<int> ck;
    while(!tr.empty()){
        int fr=tr.front().first,bk=tr.front().second;
        // cout << "fr: " << fr << " bk: " << bk << endl;
        tr.pop();
        if(ck.find(fr)!=ck.end()){
            cout << -1 << endl;
            return 0;
        }
        ck.insert(fr);
        swap(p[fr],p[bk]);
        if(p[fr]<fr){
            ud[fr]=-1;
        }else if(p[fr]==fr){
            ud[fr]=0;
        }
        if(p[bk]>bk){
            ud[bk]=1;
        }else if(p[bk]==bk){
            ud[bk]=0;
        }
        if(fr!=0 && ud[fr]==-1 && ud[fr-1]==1){
            tr.push(make_pair(fr-1,fr));
        }
        if(bk!=n-1 && ud[bk]==1 && ud[bk+1]==-1){
            tr.push(make_pair(bk,bk+1));
        }
        ans.push(fr+1);
        // cout << "reached" << endl;
    }
    for(int i=0;i<n;i++){
        if(ud[i]!=0){
            cout << -1 << endl;
            return 0;
        }
    }
    if(ck.size()!=n-1){
        cout << -1 << endl;
        return 0;
    }
    while(!ans.empty()){
        cout << ans.front() << endl;
        ans.pop();
    }
    return 0;
}
