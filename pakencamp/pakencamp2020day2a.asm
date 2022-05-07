      bits        64
      extern      scanf
      extern      printf

      section     .text
      global      main
main:
      push        rbp
      mov         rbp,    rsp
      lea         rsi,    [n]
      lea         rdx,    [m]
      lea         rdi,    [sfmt]
      call        scanf
      mov         edx,    [n]
      mov         esi,    edx
      cmp         edx,    [m]
      jns         .1
      mov         esi,    [m]
.1:
      add         edx,    [m]
      lea         rdi,    [pfmt]
      call        printf
      pop         rbp
      ret

      section     .data
sfmt: db          "%d %d", 0
pfmt: db          "%d %d", 10, 0

      section     .bss
n:    resd        1
m:    resd        1