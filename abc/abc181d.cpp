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

    string s;
    cin >> s;
    set<int> check;
    vector<int> num(10,0);
    for(int i=0;i<s.size();i++){
        check.insert(s[i]-'0');
        num[s[i]-'0']++;
    }
    if(s.size()==1){
        if(check.find(8)!=check.end()){
            cout << "Yes" << endl;
            return 0;
        }else{
            cout << "No" << endl;
            return 0;
        }
    }else if(s.size()==2){
        int i,j;
        i=stoi(s);
        reverse(s.begin(),s.end());
        j=stoi(s);
        if(i%8==0){
            cout << "Yes" << endl;
            return 0;
        }else if(j%8==0){
            cout << "Yes" << endl;
            return 0;
        }else{
            cout << "No" << endl;
            return 0;
        }
    }else{
        bool flag=false;
        for(int i=1;i<10;i++){
            if(num[i]==0)continue;
            for(int j=1;j<10;j++){
                if(num[j]==0)continue;
                if(i==j && num[j]<2)continue;
                for(int k=1;k<10;k++){
                    if(num[k]==0)continue;            
                    if(i==k && num[k]<2)continue;
                    if(j==k && num[k]<2)continue;
                    if(i==k && j==k && num[k]<3)continue;
                    int n,rn;
                    n=10*j+k;
                    rn=10*k+j;
                    if(i%2==0){
                        if(n%8==0 || rn%8==0){
                            flag=true;
                        }
                    }else{
                        if((n-4)%8==0 || (rn-4)%8==0){
                            flag=true;
                        }
                    }
                }
            }
        }
        if(flag)cout << "Yes" << endl;
        else cout << "No" << endl;
    }

    return 0;
}
