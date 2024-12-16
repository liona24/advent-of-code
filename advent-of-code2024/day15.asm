[bits 64]

%include "lib.asm"

%define MAX_SIZE 64
%define DIR_BUF_SIZE 0x4000

global _start
_start:
    lea rbp, [rel grid]

    ; width
    xor r12d, r12d
    ; height
    xor r13d, r13d

    .next_line:
    mov rdi, rbp
    mov esi, MAX_SIZE
    call read_line

    cmp rax, 0
    jl .exit
    je .solve

    or r12d, eax
    cmp r12d, eax
    jne .exit

    add r13d, 1
    add rbp, MAX_SIZE
    jmp .next_line

    .solve:

    lea rbp, [rel grid]
    xor esi, esi

    mov al, '@'

    .find_robot:
    mov ecx, r12d
    mov rdi, rbp
    repne scasb
    jz .found_robot

    add rbp, MAX_SIZE
    add esi, 1
    cmp esi, r13d
    jae .exit
    jmp .find_robot

    .found_robot:
    ; robot position
    mov r15, rdi
    sub r15, 1

    lea rbp, [rel directions]

    .read_directions:
    mov rdi, rbp
    mov esi, DIR_BUF_SIZE
    call read_line

    add rbp, rax

    cmp rax, 0
    jl .exit
    jne .read_directions

    lea rbp, [rel directions]
    .next_direction:

    xor r8d, r8d
    mov al, byte [rbp]

    mov rdx, 1
    cmp al, '>'
    cmove r8, rdx
    neg rdx
    cmp al, '<'
    cmove r8, rdx
    mov rdx, MAX_SIZE
    cmp al, 'v'
    cmove r8, rdx
    neg rdx
    cmp al, '^'
    cmove r8, rdx

    test r8, r8
    jz .print_solution

    .try_move:
    lea rdi, [r15 + r8]
    mov al, byte [rdi]
    cmp al, 'O'
    je .try_push
    cmp al, '#'
    je .next_direction.continue
    jmp .next_direction.update

    .try_push:
    mov rsi, rdi

    .try_push.more:
    add rsi, r8
    mov al, byte [rsi]
    cmp al, 'O'
    je .try_push.more
    cmp al, '#'
    je .next_direction.continue

    mov al, 'O'
    mov byte [rsi], al
    xor byte [rdi], al

    .next_direction.update:
    mov r15, rdi
    .next_direction.continue:
    add rbp, 1
    jmp .next_direction

    .print_solution:

    xor r15d, r15d
    xor ebx, ebx
    mov r8d, 100
    lea rbp, [rel grid]

    .agg.line:
    mov rdi, rbp
    mov ecx, r12d

    .agg.col:
    mov al, 'O'
    repne scasb
    jnz .agg.next_line

    mov rax, r15
    mul r8
    add rax, r12
    sub rax, rcx
    sub rax, 1
    add rbx, rax

    jmp .agg.col

    .agg.next_line:
    add rbp, MAX_SIZE
    add r15d, 1
    cmp r15d, r13d
    jl .agg.line

    mov rdi, rbx
    call print_number
    .exit:
    mov edi, eax
    call exit


section .bss
grid: resb (MAX_SIZE*MAX_SIZE)
directions: resb DIR_BUF_SIZE
