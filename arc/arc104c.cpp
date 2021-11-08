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
    vector<pii> ab(n);
    vector<int> onoff(2*n,0);
    set<int> check;
    bool ans=true;
    for(auto &x:ab){
        cin >> x.first >> x.second;
        if(x.second!=-1 && x.first!=-1 && x.first>=x.second){
            ans=false;
        }
        if(check.find(x.first-1)!=check.end() || check.find(x.second-1)!=check.end()){
            ans=false;
        }
        if(x.second!=-1 && x.first!=-1 && x.second-x.first>2){
            ans=false;
        }
        if(x.first!=-1){
            x.first--;
            onoff[x.first]++;
            check.insert(x.first);
        }
        if(x.second!=-1){
            x.second--;
            onoff[x.second]++;
            check.insert(x.second);
        }
    }
    for(int i=0;i<n;i++){
        int s=ab[i].first,e=ab[i].second;
        if(s!=-1 && e!=-1)continue;
        if(s==-1 && e==-1)continue;
        if(s==-1){
            if(e>1 && onoff[e-2]==0){
                onoff[e-2]++;
            }else if(e>0 && onoff[e-1]==0){
                onoff[e-1]++;
            }else{
                ans=false;
                break;
            }
        }else if(e==-1){
            if(s<2*n-2 && onoff[s+2]==0){
                onoff[s+2]++;
            }else if(s<2*n-1 && onoff[s+1]==0){
                onoff[s+1]++;
            }else{
                ans=false;
                break;
            }
        }
    }

    int flag=0,flagpt=0;
    for(int i=0;i<2*n-1;i++){
        if(onoff[i]!=-1)continue;
        else{
            if(flag==1){
                if(flagpt-i>2){
                    ans=false;
                    break;
                }else{
                    flag=0;
                }
            }else if(flag==0){
                flag=1;
                flagpt=i;
            }
        }
    }

    if(ans)cout << "Yes" << endl;
    else cout << "No" << endl;

    return 0;
}
