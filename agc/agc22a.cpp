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
    vector<int> c(26,0);
    for(int i=0;i<s.size();i++){
        c[s[i]-'a']++;
    }
    if(s.size()<26){
        for(int i=0;i<26;i++){
            if(c[i]==0){
                s+='a'+i;
                break;
            }
        }
        cout << s << endl;
    }else{
        string ans;
        for(int i=s.size()-1;i>=0;i--){
            for(int j=s[i]-'a';j<26;j++){
                if(!c[j]){
                    for(int k=0;k<i;k++)
                        cout << s[k];
                    cout << (char)(j+'a');
                    return 0;
                }
            }
            c[s[i]-'a']--;
        }
        cout << -1 << endl;
    }
    return 0;
}
