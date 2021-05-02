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

#define bil ((ll)1e9)
#define mod ((ll)1e9+7)

int main(int argc,char* argv[]){
    vector<int> x(5);
    for(auto &a:x)
        cin >> a;
    
    for(int i=0;i<5;i++){
        if(x.at(i)==0){
            cout << i+1 << endl;
            return 0;
        }
    }

    return 0;
}
