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
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
    int n;
    cin >> n;

    vector<pair<string,int>> book(0),list(0);
    for(int i=0;i<n;i++){
        string s;
        int p;
        cin >> s >> p;
        list.push_back(make_pair(s,p));
        book.push_back(make_pair(s,100-p));
    }

    sort(book.begin(),book.end());

    for(int i=0;i<n;i++){
        for(int j=0;j<n;j++){
            if(list[j].first==book[i].first && list[j].second==100-book[i].second){
                cout << j+1 << endl;
                break;
            }
        }
    }

    return 0;
}
