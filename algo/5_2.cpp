#include<iostream>
#include<vector>

using namespace std;

int main() {
    int n,w;
    cin >> n >> w;
    vector<int> a(n);
    for(int i=0;i<n;i++) cin >> a[i];
    vector<vector<bool>> dp(n,vector<bool>(w+1, false));
    for(int i=0;i<n;i++) dp[i][0]=true;
    for(int i=0;i<n;i++){
        for(int j=0;j<w+1;j++){
            if(j-a[i]>=0) dp[i][j]=dp[i][j-a[i]];
        }
    }
    bool ans=false;
    for(int i=0;i<n;i++) if(dp[i][w]) ans=true;
    if(ans) cout << "Yes" << endl;
    else cout << "No" << endl;
}