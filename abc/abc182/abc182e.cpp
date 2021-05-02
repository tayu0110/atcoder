#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>

using namespace std;

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

// const int MAX=1505;
// int di[4] = {1, 0, -1, 0}, dj[4] = {0, 1, 0, -1};
// bool reached[MAX][MAX];
// bool check[MAX][MAX];
// bool memo[MAX][MAX];
// int field[MAX][MAX];
// int h,w;

// bool f(int v, int i, int j){
//     if(i<0 || j<0 || i>=h || j>=w)return false;
//     if(field[i][j]==1)return true;
//     if(field[i][j]==-1)return false;
//     if(reached[i][j])return memo[i][j];
//     reached[i][j]=true;
//     return memo[i][j]=f(v, i+di[v], j+dj[v]);
// }

// int main(){
//     int n,m;
//     cin >> h >> w >> n >> m;
//     for(int i=0;i<n;i++){
//         int a,b;
//         cin >> a >> b;
//         a--;b--;
//         field[a][b]=1;
//     }
//     for(int i=0;i<m;i++){
//         int c,d;
//         cin >> c >> d;
//         c--;d--;
//         field[c][d]=-1;
//     }
//     for(int v=0;v<4;v++){
//         for(int i=0;i<h;i++)for(int j=0;j<w;j++)reached[i][j]=false;
//         for(int i=0;i<h;i++)for(int j=0;j<w;j++)if(f(v,i,j))check[i][j]=true;
//     }
//     // test
//     // for(int i=0;i<h;i++){
//     //     for(int j=0;j<w;j++){
//     //         cout << check[i][j] << " ";
//     //     }
//     //     cout << endl;
//     // }
//     ll ans=0;
//     for(int i=0;i<h;i++)for(int j=0;j<w;j++)if(check[i][j])ans++;
//     cout << ans << endl;
//     return 0;
// }

queue<pair<pii,pii>> ptr,ptc;

int main(int argc,char* argv[]){
    cin.tie(0);
    ios::sync_with_stdio(0);
    cout << fixed << setprecision(20);

    int h,w,n,m;
    cin >> h >> w >> n >> m;
    vector<int> a(n),b(n),c(m),d(m);
    vector<vector<int>> field(h+1,vector<int>(w+1,0));
    for(int i=0;i<n;i++){
        cin >> a[i] >> b[i];
        int y=a[i],x=b[i];
        field[y][x]=1;
    }
    for(int i=0;i<m;i++){
        cin >> c[i] >> d[i];
        int y=c[i],x=d[i];
        field[y][x]=-1;
    }

    for(int i=1;i<h+1;i++){
        int bf=0;
        bool lf=false;
        for(int j=1;j<w+1;j++){
            if(!lf){
                if(field[i][j]==1){
                    lf=true;
                }else if(field[i][j]==-1){
                    bf=j;
                }else continue;
            }else{
                if(field[i][j]==1)continue;
                else if(field[i][j]==-1){
                    ptr.push(make_pair(make_pair(i,bf+1),make_pair(i,j)));
                    bf=j;
                    lf=false;
                }else continue;
            }
        }
        if(lf){
            ptr.push(make_pair(make_pair(i,bf+1),make_pair(i,w+1)));
        }
    }
    ll count=0;
    vector<vector<bool>> check(h+1,vector<bool>(w+1,false));
    while(!ptr.empty()){
        pii st=ptr.front().first,ed=ptr.front().second;
        int stx=st.second,sty=st.first,edx=ed.second;
        // cout << "now: " << stx << " " << sty << " " << edx << " " << edy << endl;
        ptr.pop();
        
        for(int i=stx;i<edx;i++){
            if(check[sty][i])continue;
            count++;
            check[sty][i]=true;
        }
    }
    for(int j=1;j<w+1;j++){
        bool lf=false;
        int bf=0;
        for(int i=1;i<h+1;i++){
            if(!lf){
                if(field[i][j]==1){
                    lf=true;
                }else if(field[i][j]==-1){
                    bf=i;
                }else continue;
            }else{
                if(field[i][j]==1)continue;
                else if(field[i][j]==-1){
                    ptc.push(make_pair(make_pair(bf+1,j),make_pair(i,j)));
                    bf=i;
                    lf=false;
                }else continue;
            }
        }
        if(lf){
            ptc.push(make_pair(make_pair(bf+1,j),make_pair(h+1,j)));
        }
    }
    while(!ptc.empty()){
        pii st=ptc.front().first,ed=ptc.front().second;
        int stx=st.second,sty=st.first,edy=ed.first;
        // cout << "now: " << stx << " " << sty << " " << edx << " " << edy << endl;
        ptc.pop();
        for(int i=sty;i<edy;i++){
            if(check[i][stx])continue;
            count++;
            check[i][stx]=true;
        }
    }

    cout << count << endl;

    return 0;
}
