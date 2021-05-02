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

struct Edge {
  int to;
  long long weight;
  Edge(int to, long long weight) : to(to), weight(weight) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

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
  int n = s.length();
  ll ans = 0;
  vector<vector<int>> dp(26, vector<int>(n+1, 0));
  for(int i=n-1;i>=0;i--){
    int num = s[i]-'a';
    if(i-1 >= 0 && s[i]==s[i-1]){
      ans += n-1-i-dp[num][i+1];
      // cout << "s[i]: " << s[i] << " ans: " << n-1-i-dp[num][i+1] << endl;
      for(int j=0;j<26;j++){
        if(j==num) dp[j][i]=n-i;
        else dp[j][i]=0;
      }
    } else {
      for(int j=0;j<26;j++){
        if(j==num) dp[j][i] = dp[j][i+1]+1;
        else dp[j][i] = dp[j][i+1];
      }
    }
  }
  cout << ans << endl;
  return 0;
}
