#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<set>
#include<stack>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using pii = pair<int, int>;

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    string s;
    cin >> s;

    int n=s.size();

    if(s[0]==s[n-1] && n%2==1){
        cout << "Second" << endl;
    }else if(s[0]==s[n-1] && n%2==0){
        cout << "First" << endl;
    }else if(s[0]!=s[n-1] && n%2==1){
        cout << "First" << endl;
    }else if(s[0]!=s[n-1] && n%2==0){
        cout << "Second" << endl;
    }

    return 0;
}
