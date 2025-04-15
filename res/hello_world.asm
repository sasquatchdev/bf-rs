
    section .bss
        mem resb 30000
    section .text
        global _start
    
    _start:
        mov rsi, mem
        inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
loop_start_0:

                    cmp byte [rsi], 0
                    je loop_end_0
                    inc rsi
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
loop_start_1:

                    cmp byte [rsi], 0
                    je loop_end_1
                    inc rsi
    inc byte [rsi]
    inc byte [rsi]
    inc rsi
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc rsi
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc rsi
    inc byte [rsi]
    dec rsi
    dec rsi
    dec rsi
    dec rsi
    dec byte [rsi]
  cmp byte [rsi], 0
  jne loop_start_1
loop_end_1:
    inc rsi
    inc byte [rsi]
    inc rsi
    inc byte [rsi]
    inc rsi
    dec byte [rsi]
    inc rsi
    inc rsi
    inc byte [rsi]
loop_start_2:

                    cmp byte [rsi], 0
                    je loop_end_2
                    dec rsi
  cmp byte [rsi], 0
  jne loop_start_2
loop_end_2:
    dec rsi
    dec byte [rsi]
  cmp byte [rsi], 0
  jne loop_start_0
loop_end_0:
    inc rsi
    inc rsi

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                inc rsi
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
            
                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                inc rsi
    inc rsi

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                dec rsi
    dec byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                dec rsi

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                inc byte [rsi]
    inc byte [rsi]
    inc byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]
    dec byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                inc rsi
    inc rsi
    inc byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
                inc rsi
    inc byte [rsi]
    inc byte [rsi]

                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
            
        mov rax, 60
        xor rdi, rdi
        syscall
    