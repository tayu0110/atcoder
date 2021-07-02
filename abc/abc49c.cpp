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

    if(s.size()<5){
        cout << "NO" << endl;
        return 0;
    }

    string ans="NO";
    int checked;
    for(checked=0;checked<=s.size();){
        if(s.size()-checked<5){
            ans="NO";
            break;
        }
        if(s.substr(checked,5)=="dream"){
            checked+=5;
            if(checked==s.size()){
                ans="YES";
                break;
            }
            if(s.substr(checked,2)=="er"){
                if(checked+2==s.size()){
                    ans="YES";
                    break;
                }else{
                    if(s.at(checked+2)=='e' || s.at(checked+2)=='d'){
                        checked+=2;
                        continue;
                    }else if(s.at(checked+2)=='a'){
                        continue;
                    }else{
                        ans="NO";
                        break;
                    }
                }
            }
        }else if(s.substr(checked,5)=="erase"){
            checked+=5;
            if(checked==s.size()){
                ans="YES";
                break;
            }
            if(s.substr(checked,1)=="r"){
                if(checked+1==s.size()){
                    ans="YES";
                    break;
                }else{
                    if(s.at(checked+1)=='e' || s.at(checked+1)=='d'){
                        checked+=1;
                        continue;
                    }else if(s.at(checked+1)=='a'){
                        continue;
                    }else{
                        ans="NO";
                        break;
                    }
                }   
            }
        }else{
            ans="NO";
            break;
        }
    }

    cout << ans << endl;

    return 0;
}
