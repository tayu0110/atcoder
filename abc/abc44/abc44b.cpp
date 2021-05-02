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

    if(s.size()%2!=0){
        cout << "No" << endl;
        return 0;
    }

    sort(s.begin(),s.end());

    for(int i = 0; i < s.size()-1; i+=2){
        if(s.at(i)==s.at(i+1)){
            continue;
        }else{
            cout << "No" << endl;
            return 0;
        }
    }

    cout << "Yes" << endl;

    return 0;
}
