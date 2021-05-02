//failed code
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

using namespace std;

using ll = long long;

int main(int argc,char* argv[]){
    int n,k;
    cin >> n >> k;

    vector<int> a(n);
    vector<int> flashed(n,0);
    for(auto x:a){
        cin >> x;
    }
    for(int h=1;h<=k;h++){
        for(int i=0;i<n;i++){
            if(a.at(i)==0){
                flashed.at(i)++;
                continue;
            }
            for(int j=i-a.at(i);j<=i+a.at(i);j++){
                if(j<0) continue;
                flashed.at(j)++;
            }
        }
        for(int l=0;l<n;l++){
            a.at(l)=flashed.at(l);
            flashed.at(l)=0;
        }
    }
    
    for(int i=0;i<n;i++){
        cout << a.at(i) << " ";
    }
    cout << endl;

    return 0;
}
