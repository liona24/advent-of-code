[bits 64]

%include "lib.asm"

%define LINE_BUF_LEN 512
%define MAX_NUM_PER_LINE 128

global _start
_start:
    ; line buffer
    sub rsp, LINE_BUF_LEN
    mov rbx, rsp

    ; accumulator of result
    xor r14, r14

    .next:
    mov rdi, rbx
    mov rsi, LINE_BUF_LEN
    call read_line

    cmp rax, 0
    jl .error
    jz .done

    ; remaining line length
    mov rsi, rax
    ; count of numbers pulled
    xor r15, r15

    mov rdi, rbx

    .pull_number:

    ; to check whether we pulled more characters
    mov rbp, rsi

    call atou

    ; rsi, rdi preserved to next number ..

    cmp rsi, rbp
    je .check_line

    lea rcx, [rel numbers]
    mov qword [rcx + r15 * 8], rax
    inc r15

    mov al, 1
    cmp r15, MAX_NUM_PER_LINE
    je .error
    jmp .pull_number

    .check_line:

    test r15, r15
    jz .done

    lea rbp, [rel numbers]
    ; prev number
    mov rdi, qword [rbp]

    ; direction count, size_t: add -1 for decreasing, add +1 for increasing
    xor r13, r13
    ; i
    xor r12, r12

    .check_next_number:
    inc r12
    cmp r12, r15
    je .acc

    mov rsi, qword [rbp + r12 * 8]
    sub rdi, rsi
    js .increasing
    jz .none
    dec r13
    jmp .none
    .increasing:
    inc r13
    .none:

    ; difference to previous <= 3?
    abs rdi, rax
    cmp rdi, 3
    ja .not_safe

    mov rdi, rsi
    jmp .check_next_number

    .not_safe:
    xor r13, r13
    .acc:

    ; all numbers decreasing or increasing?
    abs r13, rdi
    inc r13
    cmp r13, r15
    jne .next
    inc r14
    jmp .next

    .done:
    mov rdi, r14
    call print_number
    cmp rax, 0
    jl .error
    xor rax, rax
    .error:
    mov rdi, rax
    call exit

section .bss
numbers: resq MAX_NUM_PER_LINE

