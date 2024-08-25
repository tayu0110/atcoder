extern  printf
extern  scanf

section .rodata
    fmt_nk:     db  "%d %d", 0
    fmt_a:      db  "%d", 0
    fmt_sp:     db  " ", 0
    fmt_ln:     db  10, 0

section .text
global main
main:
    push    rbp
    sub     rsp,    16
    lea     rdi,    [rel fmt_nk]
    lea     rsi,    [rsp]
    lea     rdx,    [rsp + 4]
    xor     eax,    eax
    call    scanf   wrt ..plt

    xor     ebp,    ebp
    xor     r14d,   r14d
    mov     r15d,   [rsp + 4]
.loop:
    lea     rdi,    [rel fmt_a]
    lea     rsi,    [rsp + 8]
    xor     eax,    eax
    call    scanf   wrt ..plt

    mov     eax,    [rsp + 8]
    xor     edx,    edx
    div     r15d

    test    edx,    edx
    jnz     .non_zero
    push    rax

    test    r14d,   r14d
    jz      .print
    lea     rdi,    [rel fmt_sp]
    xor     eax,    eax
    call    printf  wrt ..plt
.print:
    pop     rsi
    lea     rdi,    [rel fmt_a]
    xor     eax,    eax
    call    printf  wrt ..plt

    inc     r14d

.non_zero:
    inc     ebp
    cmp     ebp,    [rsp]
    jnz     .loop

    lea     rdi,    [rel fmt_ln]
    xor     eax,    eax
    call    printf  wrt ..plt

    add     rsp,    16
    pop     rbp
    xor     eax,    eax
    ret
