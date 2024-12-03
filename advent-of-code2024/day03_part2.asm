[bits 64]

%include "lib.asm"

%define BUF_LEN 0x8000

global _start
_start:
    lea rdi, [rel buffer]
    mov rsi, BUF_LEN
    call read_line

    cmp rax, 0
    jl .done

    ; input length
    mov rbx, rax

    ; a simple repeat loop for performance evaluation
    ; xor r12, r12
    ; push r12
    ; .rep_loop:
    ; cmp qword [rsp], 0xF000
    ; je .final
    ; inc qword [rsp]
    ;
    ; Unoptimized, we hit 0.7 GiByte / sec for the current implementation

    lea rbp, [rel buffer]

    ; enabled
    mov r15b, 1

    ; result
    xor r13, r13

    ; i
    xor r12, r12

    .next:
    mov rdi, r12
    add rdi, 8
    cmp rdi, rbx
    ja .result

    mov rdi, qword [rbp + r12]
    inc r12

    test r15b, r15b
    ; for the first part, just replace jnz with jmp
    jnz .enabled
    cmp edi, "do()"
    sete r15b
    jmp .next

    .enabled:
    cmp edi, "mul("
    je .parse_mul_0

    shl rdi, 8
    mov rax, `\0don't()`
    cmp rdi, rax
    sete dil
    xor r15b, dil
    jmp .next

    .parse_mul_0:
    add r12, 3

    ; number to parse
    xor rax, rax
    ; base
    mov r8, 10

    mov cl, 4

    .next_char_0:
    mov dil, byte [rbp + r12]

    cmp dil, ','
    je .parse_mul_1

    dec cl
    jz .next

    cmp dil, '0'
    jb .next
    cmp dil, '9'
    ja .next

    sub dil, '0'
    movzx rdi, dil
    mul r8
    add rax, rdi

    inc r12
    jmp .next_char_0

    .parse_mul_1:

    cmp cl, 4
    je .next

    mov r14, rax
    xor rax, rax

    mov cl, 4

    .next_char_1:
    inc r12
    cmp r12, rbx
    je .result

    mov dil, byte [rbp + r12]

    cmp dil, ')'
    je .aggregate

    dec cl
    jz .next

    cmp dil, '0'
    jb .next
    cmp dil, '9'
    ja .next

    sub dil, '0'
    movzx rdi, dil
    mul r8
    add rax, rdi

    jmp .next_char_1

    .aggregate:

    cmp cl, 4
    je .next

    mul r14
    add r13, rax

    inc r12
    jmp .next

    .result:
    ; enable for performance loop
    ; jmp .rep_loop

    .final:
    mov rdi, r13
    call print_number

    cmp rax, 0
    jl .done
    xor rax, rax
    .done:
    mov rdi, rax
    call exit

section .bss
buffer: resb BUF_LEN

