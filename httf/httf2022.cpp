#include <iostream>
#include <vector>
#include <set>
#include <queue>
#include <random>

using namespace std;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;

const int inf = 1 << 29;
const int init_skill = inf;

int n, m, k, r;
int now_time;

static ld randnormal() {
  random_device seed_gen;
  default_random_engine engine(seed_gen());
  normal_distribution<ld> dist((ld)0.0, (ld)1.0);
  return dist(engine);
}
static int randint(int L, int U) {
  random_device seed_gen;
  default_random_engine engine(seed_gen());
  uniform_int_distribution<> dist(L, U);
  return dist(engine);
}
static ld randdouble(ld L, ld U) {
  random_device seed_gen;
  default_random_engine engine(seed_gen());
  uniform_real_distribution<ld> dist(L, U);
  return dist(engine);
}
static vector<ld> randnormal_vec(int sz) {
  vector<ld> res(sz);
  for(int i=0;i<sz;i++) res[i] = randnormal();
  return res;
}
static vector<int> randint_vec(int L, int U, int sz) {
  vector<int> res(sz);
  for(int i=0;i<sz;i++) res[i] = randint(L, U);
  return res;
}
static vector<ld> randdouble_vec(ld L, ld U, int sz) {
  vector<ld> res(sz);
  for(int i=0;i<sz;i++) res[i] = randdouble(L, U);
  return res;
}

struct Task {
  int id;
  int dep;
  vector<int> req_lev;
  Task() : id{-1} {}
  Task(int _id) : id{_id}, dep{0}, req_lev{vector<int>(k)} {}
  Task(const Task &t) : id{t.id}, dep{t.dep}, req_lev{t.req_lev} {}
  bool operator<(const Task &rhs) const { return dep < rhs.dep; }
  bool operator>(const Task &rhs) const { return dep > rhs.dep; }
  void init() {
    for(int i=0;i<k;i++) cin >> req_lev[i];
  }
  void set_dep(int _dep) { dep = _dep; }
  void dec_dep(int dec) { dep -= dec; }
  int get_dep() { return dep; }
  vector<int> &get_req_lev() { return req_lev; }
};
struct Tasks {
  vector<Task> tasks;
  vector<set<int>> dep, rev_dep;
  priority_queue<Task> enable_que;
  Tasks() {}
  void init() {
    tasks.resize(n);
    for(int i=0;i<n;i++) {
      Task t(i);
      t.init();
      tasks[i] = t;
    }
    dep.resize(n);
    rev_dep.resize(n);
    for(int i=0;i<r;i++) {
      int u, v;
      cin >> u >> v;
      u--; v--;
      dep[u].insert(v);
      rev_dep[v].insert(u);
    }
    for(int i=0;i<n;i++) {
      tasks[i].set_dep(dep[i].size());
      if(!rev_dep[i].size()) enable_que.push(tasks[i]);
    }
  }
  Task get_task(int id) { return tasks[id]; }
  bool can_assign() { return !enable_que.empty(); }
  void resolve(int id) {
    for(auto d : dep[id]) {
      rev_dep[d].erase(id);
      if(rev_dep[d].empty()) enable_que.push(d);
    }
  }
  Task assign() {
    Task t = enable_que.top();
    enable_que.pop();
    return t;
  }
};
Tasks tasks;

struct Worker {
  int id;
  int task_id;
  int start_time;
  int experience;
  vector<int> level;
  Worker() : id{-1}, task_id{-1}, start_time{0}, experience{0} {}
  Worker(int _id) : id{_id}, task_id{-1}, start_time{0}, experience{0} {}
  Worker(const Worker &w) : id{w.id}, task_id{w.task_id}, start_time{w.start_time}, experience{0}, level{w.level} {}
  bool operator<(const Worker &rhs) const { return id < rhs.id; }
  void init() {
    level = gen_init_level();
  }
  void set_task(Task task) { task_id = task.id; start_time = now_time; }
  void erase_task() { task_id = -1; }
  vector<int> get_level() { return level; }
  void evaluate() {
    experience++;
    int tdiff = now_time - start_time;
    tdiff = max(1, tdiff - randint(-3, 3));
    Task task = tasks.get_task(task_id);
    vector<int> diff = task.get_req_lev();
    auto f = (tdiff == 1 ? [](int l, int r) { return l < r; } : [](int l, int r) { return l > r; });
    for(int i=0;i<k;i++) if(f(level[i], diff[i])) level[i] = (level[i] * experience + diff[i]) / (experience + 1);
    // if(tdiff > 1) parameter_adjust(diff, tdiff);
  }
 private:
  vector<int> gen_init_level() {
    vector<ld> s = randnormal_vec(k);
    ld ssum = 0;
    for(int i=0;i<k;i++) ssum += s[i] * s[i];
    ssum = sqrtl(ssum);
    vector<ld> p(k);
    for(int i=0;i<k;i++) p[i] = randdouble(20, 60) / ssum;
    vector<int> res(k);
    for(int i=0;i<k;i++) res[i] = round(p[i] * s[i]);
    return res;
  }
  void parameter_adjust(vector<int> &diff, int tdiff) {
    int w = 0;
    for(int i=0;i<k;i++) w += max(0, diff[i] - level[i]);
    int cnt = 0;
    while(w != tdiff) {
      if(w < tdiff) level[cnt]--;
      else level[cnt]++;
      cnt = (cnt+1) % k;
      w = 0;
      for(int i=0;i<k;i++) w += max(0, diff[i] - level[i]);
    }
  }
};
struct Workers {
  vector<Worker> workers;
  multiset<Worker> free_worker;
  Workers() {}
  void init() {
    workers.resize(m);
    for(int i=0;i<m;i++) {
      Worker w(i);
      w.init();
      workers[i] = w;
      free_worker.insert(w);
    }
  }
  Worker &get_worker(int id) { return workers[id]; }
  bool can_assign() { return !free_worker.empty(); }
  Worker assign(Task task) {
    auto assigned = free_worker.begin();
    int tmin = inf;
    vector<int> &diff = task.get_req_lev();
    for(auto it=free_worker.begin();it!=free_worker.end();it++) {
      Worker w = *it;
      vector<int> level = w.get_level();
      int tnow = 0;
      for(int i=0;i<k;i++) tnow += max(0, diff[i] - level[i]);
      if(tnow < tmin) {
        tmin = tnow;
        assigned = it;
      }
    }
    int id = assigned->id;
    free_worker.erase(assigned);
    workers[id].set_task(task);
    return workers[id];
  }
  void resolve(int id) {
    int task_id = workers[id].task_id;
    tasks.resolve(task_id);
    workers[id].evaluate();
    workers[id].erase_task();
    free_worker.insert(workers[id]);
  }
};
Workers workers;

struct Solver {
  void init() {
    tasks.init();
    workers.init();
  }
  vector<Worker> assign() {
    vector<Worker> res;
    while(workers.can_assign() && tasks.can_assign()) {
      Task task = tasks.assign();
      Worker worker = workers.assign(task);
      res.push_back(worker);
    }
    return res;
  }
  void send() {
    vector<Worker> res = assign();
    printf("%d", (int)res.size());
    for(auto w : res) printf(" %d %d", w.id+1, w.task_id+1);
    printf("\n");
    fflush(stdout);
  }
  int receive() {
    int rec;
    scanf("%d", &rec);
    DEBUG_EN(rec);
    for(int i=0;i<rec;i++) {
      int f;
      scanf("%d", &f);
      f--;
      DEBUG(i);DEBUG(f);DEBUG_EN("reached");
      workers.resolve(f);
    }
    return rec;
  }
  int solve() {
    now_time++;
    send();
    return receive();
  }
};

int main(int argc, char *argv[]) {
  scanf("%d %d %d %d", &n, &m, &k, &r);
  Solver solver;
  solver.init();
  while(solver.solve() >= 0);
  return 0;
}