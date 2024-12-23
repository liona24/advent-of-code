[bits 64]

%include "lib.asm"

global _start
_start:
    xor r15, r15
    sub rsp, 16

    .read_next:
    mov rdi, rsp
    mov esi, 16
    call read_line

    cmp rax, 0
    jl .exit
    je .done

    mov esi, eax
    mov rdi, rsp
    call atou

    mov rdi, rax
    mov rax, (16777216 - 1)

    xor ecx, ecx

    .next_number:
    mov rsi, rdi
    shl rsi, 6
    xor rdi, rsi
    and rdi, rax

    mov rsi, rdi
    shr rsi, 5
    xor rdi, rsi
    and rdi, rax

    mov rsi, rdi
    shl rsi, 11
    xor rdi, rsi
    and rdi, rax

    add ecx, 1
    cmp ecx, 2000
    jne .next_number

    add r15, rdi
    jmp .read_next

    .done:
    mov rdi, r15
    call print_number

    xor eax, eax
    .exit:
    mov edi, eax
    call exit


