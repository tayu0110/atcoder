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
    string s;
    cin >> s;

    if(s.size()==2){
        if(s.at(0)==s.at(1)){
            cout << "1" << " "  << "2" << endl;
            return 0;
        }else{
            cout << "-1" << " " << "-1" << endl;
            return 0;
        }
    }

    for(int i=0;i<s.size()-2;i++){
        if(s.at(i)==s.at(i+1)){
            cout << i+1 << " " << i+2 << endl;
            return 0;
        }
        if(s.at(i)==s.at(i+2)){
            cout << i+1 << " " << i+3 << endl;
            return 0;
        }
    }

    cout << "-1" << " " << "-1"  << endl;

    return 0;
}
