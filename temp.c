#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

typedef long long ll;
typedef long double ld;

#define MAX_LIST_LEN 10000000

const int inf = 1 << 29;
const ll INF = 1LL << 60;
const ll MOD = 1e9+7;

typedef struct {
  int to; ll weight;
} Edge;

struct L {
  int element;
  struct L* prev;
  struct L* next;
};
typedef struct L List;
List* init_list() {
  List* node = (List*)malloc(sizeof(List));
  node->element = 0;
  node->prev = NULL;
  node->next = NULL;
  return node;
}
void insert_list(List* list, int element, int pos) {
  if(list == NULL) return;
  List* node = list;
  List* prev = NULL;
  for(int i = 0; i < pos && node != NULL; i++, node = node->next) prev = node;
  List* new_node = (List*)malloc(sizeof(List));
  new_node->element = element;
  new_node->next = node;
  new_node->prev = prev;
  if(prev) prev->next = new_node;
  if(node) node->prev = new_node;
}
void insert_last_list(List* list, int element) {
  insert_list(list, element, MAX_LIST_LEN);
}
void insert_head_list(List* list, int element) {
  insert_list(list, element, 0);
}
/*void erase_list(List** list, int pos) {
  if(list == NULL) return;
  List* node = *list;
  while(pos--) {
    node = node->next;
    if(!node) return;
  }
  if(node->prev) node->prev->next = node->next;
  if(node->next) node->next->prev = node->prev;
  if(!node->prev && !node->next) {
    free(*list);
    *list = NULL;
  } else {
    free(node);
    node = NULL;
  }
}*/
void erase_list(List* list, int pos) {
  if(list == NULL) return;
  List* node = list;
  while(pos--) {
    node = node->next;
    if(!node) return;
  }
  if(node->prev) node->prev->next = node->next;
  if(node->next) node->next->prev = node->prev;
    free(node);
    node = NULL;
}

int main(int argc, char* argv[]) {
  List* list = init_list();
  list->element = 1;
  for(int i = 2; i <= 10; i++) {
    insert_last_list(list, i);
  }
  List* itr = list;
  do {
    printf("%d\n", itr->element);
  } while(itr = itr->next);
  // for(int i=9;i>=0;i--) erase_list(&list, i);
  for(int i=9;i>=0;i--) erase_list(list, i);
  if(!list) {
    printf("List is deleted.\n");
  } else {
    itr = list;
    do{
      printf("%d\n", itr->element);
    } while(itr = itr->next);
  }
  return 0;
}