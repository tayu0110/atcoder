#include<stdio.h>

int main(){
    FILE *fp;

    fp=fopen("test.txt","w");

    if(fp==NULL){
        printf("�t�@�C���̃I�[�v���Ɏ��s���܂���\n");
        return -1;
    }else{
        printf("�t�@�C���͐���ɃI�[�v������܂���\n");
    }

    fprintf(fp,"���߂Ẵt�@�C���I�[�v��\n");

    if(fclose(fp)!=0){
        perror("�t�@�C���̃N���[�Y�Ɏ��s���܂���\n");
    }else{
        printf("�t�@�C���͐���ɃN���[�Y����܂���\n");
    }

    return 0;
}