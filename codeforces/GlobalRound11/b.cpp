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

    int t;
    cin >> t;
    for(int i=0;i<t;i++){
        int n,k;
        cin >> n >> k;
        string s;
        cin >> s;
        bool firstwin=false;
        int Lcount=0,Lsum=0,point=0;
        vector<int> Linspace(0);
        for(int j=0;j<n;j++){
            if(firstwin && s[j]=='L'){
                Lcount++;
                Lsum++;
            }else if(firstwin && s[j]=='W' && Lcount!=0){
                Linspace.push_back(Lcount);
                Lcount=0;
                if(j==0 || s[j-1]=='L'){
                    point++;
                }else{
                    point+=2;
                }
            }else if(firstwin && s[j]=='W' && Lcount==0){
                if(j==0 || s[j-1]=='L'){
                    point++;
                }else{
                    point+=2;
                }
            }
            if(!firstwin && s[j]=='W'){
                firstwin=true;
                if(j==0 || s[j-1]=='L'){
                    point++;
                }else{
                    point+=2;
                }
            }
            else if(!firstwin && s[j]=='L')Lsum++;
        }

        // cout << "point:"<<point << endl;

        if(Lsum<=k){
            cout << 1+(n-1)*2 << endl;
        }else{
            // cout<< "reached" << endl;
            if(Linspace.size()>0){
                sort(Linspace.begin(),Linspace.end());
                for(int j=0;(k-Linspace[j]>=0 && j<Linspace.size());j++){
                    k-=Linspace[j];
                    Lsum-=Linspace[j];
                    point+=2*Linspace[j]+1;
                }
            }
            if(Lsum>0)point+=2*k;
            cout << point << endl;
        }
    }

    return 0;
}
