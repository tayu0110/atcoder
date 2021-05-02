#include<stdio.h>
#include<stdlib.h>

struct cell{
    int element;
    struct cell *next;
};

struct cell *insert(int x, struct cell *p, struct cell *init){
    struct cell *q,*r;

    r=(struct cell *)malloc(sizeof(struct cell));
    if(p==NULL){
        q=init;
        init=r;
    }else{
        q=p->next;
        p->next=r;
    }
    r->element=x;
    r->next=q;
    return (init);
}

struct cell *erase(struct cell *p, struct cell *init){
    struct cell *q;
    
    if(init==NULL){
        printf("Error: List is empty.\n");
        exit(1);
    }
    if(p==NULL){
        q=init;
        init=init->next;
        free(q);
    }else{
        if(p->next==NULL){
            printf("Error: No element to remove.\n");
            exit(1);
        }else{
            q=p->next;
            p->next=p->next;
            free(q);
        }
    }
    return (init);
}
