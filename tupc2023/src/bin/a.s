%define MAX 200001

global main
extern printf
extern scanf

section .rodata
    fmtn:       db      "%d", 0
    fmt:        db      "%d %d %ld", 0
    fmt_res:    db      "%ld", 10, 0

section .bss
    a:          resq    MAX
    b:          resq    MAX
    xyp:        resq    MAX * 2

section .text
main:
    push        rbp
    sub         rsp,    32
    
    lea         rdi,    [rel fmtn]
    lea         rsi,    [rsp]
    xor         eax,    eax
    call        scanf   wrt ..plt

    xor         ebx,    ebx
    lea         rbp,    [rel xyp]
    mov         r13d,   DWORD [rsp]
    shl         r13,    4
    lea         r14,    [rel a]
    lea         r15,    [rel b]
.read:
    lea         rdi,    [rel fmt]
    lea         rsi,    [rbp]
    lea         rdx,    [rbp + 4]
    lea         rcx,    [rbp + 8]
    xor         eax,    eax
    call        scanf   wrt ..plt

    movdqu      xmm0,   [rbp]
    movq        r12,    xmm0
    pextrq      r10,    xmm0,   1
    mov         r11d,   r12d
    add         QWORD [r14 + r11 * 8],  r10
    shr         r12,    32
    add         QWORD [r15 + r12 * 8],  r10

    add         ebx,    16
    add         rbp,    16
    cmp         ebx,    r13d
    jne         .read

    lea         rbp,    [rel xyp]
    xor         ebx,    ebx
    xor         esi,    esi
.solve:
    movdqu      xmm0,   [rbp]
    movq        r11,    xmm0
    pextrq      r10,    xmm0,   1
    mov         r12d,   r11d
    shr         r11,    32

    mov         rax,    QWORD [r14 + r12 * 8]
    add         rax,    QWORD [r15 + r11 * 8]
    sub         rax,    r10

    cmp         rax,    rsi
    jle         .less
    mov         rsi,    rax
.less:

    add         ebx,    16
    add         rbp,    16
    cmp         ebx,    r13d
    jne         .solve

    lea         rdi,    [rel fmt_res]
    xor         eax,    eax
    call        printf  wrt ..plt

    add         rsp,    32
    pop         rbp
    xor         eax,    eax
    ret