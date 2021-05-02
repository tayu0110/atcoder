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

    string str;
    for(int i=0;i<s.size();i++){
        if(s.at(i)=='0')
            str.push_back('0');
        else if(s.at(i)=='1')
            str.push_back('1');
        else if(s.at(i)=='B')
            if(str.size()==0)
                continue;
            else
                str.erase(str.end()-1);
    }

    cout << str << endl;

    return 0;
}
