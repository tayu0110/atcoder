//Union-Find木による無向グラフの実装
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

void unite(int, int, vector<int>&, vector<int>&);
bool same(int, int, vector<int>&);
int root(int, vector<int>&);

int main(int argc,char* argv[]){
    int n,q;
    cin >> n >> q;

    vector<int> v(n);               //各要素の親
    for(int i=0;i<v.size();i++)
        v.at(i)=i;

    vector<int> h(n,0);             //各要素の高さ
    
    int p,a,b;
    for(int i=0;i<q;i++){
        cin >> p >> a >> b;
        if(p==0){
            unite(a,b,v,h);
        }else if(p==1){
            if(same(a,b,v))
                cout << "Yes" << endl;
            else
                cout << "No" << endl;
        }
    }

    return 0;
}

//根を探索する
int root(int vertex,vector<int> &vtx){
    if(vtx[vertex]==vertex)
        return vertex;
    else{
        vtx[vertex]=root(vtx[vertex],vtx);  //経路圧縮
        return vtx[vertex];
    }
}

//木をつなげる
void unite(int a,int b,vector<int> &vtx,vector<int> &hight){
    int x=root(a,vtx);
    int y=root(b,vtx);

    if(x==y)
        return;
    //高いほうの木の根に、低い木をつなげる
    if(hight[x]>hight[y]){
        vtx[y]=x;
    }else if(hight[x]<hight[y]){
        vtx[x]=y;
    }else{
        vtx[x]=y;
        hight[y]++;
    }
}

//同じ木の要素かを判断する
bool same(int a,int b,vector<int> &vtx){
    int x=root(a,vtx);
    int y=root(b,vtx);

    if(x==y)
        return true;
    else
        return false;
}
