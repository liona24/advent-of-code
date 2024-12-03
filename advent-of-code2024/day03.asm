[bits 64]

%include "lib.asm"

%define BUF_LEN 0x8000

global _start
_start:
    lea rbp, [rel buffer]
    mov rdi, rbp
    mov rsi, BUF_LEN
    call read_line

    cmp rax, 0
    jl .done

    ; input length
    mov rbx, rax
    ; i
    xor r12, r12
    ; result
    xor r13, r13

    ; We do the following:
    ; - scan the input for a comma ','
    ; - then, try to parse the previous 1-3 digits
    ; - then, check if those are preceeded by a "mul(" prefix
    ; - then, parse the following 1-3 digits and check if there is a final
    ;    ')'

    .state.comma:
    cmp r12, rbx
    jae .result

    lea rdi, [rbp + r12]
    mov rcx, rbx
    sub rcx, r12
    mov al, ','
    repne scasb

    jnz .result

    ; outer continuation point
    mov r12, rdi
    sub r12, rbp

    ; clear numbers
    xor eax, eax
    mov [rel number0], eax
    mov [rel number1], eax

    ; address of ','
    dec rdi

    ; extract first digits
    lea rdx, [rel number0]
    mov rax, 4 ; maximum of 3 digits + 1
    .first_number.next_digit:
    dec rdi
    cmp rdi, rbp
    jb .state.comma

    mov cl, byte [rdi]
    cmp cl, '('
    je .state.check_prefix

    dec al
    jz .state.comma

    cmp cl, '0'
    jb .state.comma
    cmp cl, '9'
    ja .state.comma

    mov byte [rdx + rax], cl

    jmp .first_number.next_digit

    .state.check_prefix:

    ; check if have seen at least one digit
    cmp al, 4
    je .state.comma

    sub rdi, 3
    cmp rdi, rbp
    jb .state.comma

    mov eax, "mul("
    cmp eax, dword [rdi]
    jne .state.comma

    ; extract second digits
    lea rdi, [rbp + r12]
    lea rdx, [rel number1]
    mov rax, 4 ; maximum of 3 digits + 1
    .second_number.next_digit:
    lea rcx, [rbp + rbx]
    cmp rdi, rcx
    jae .result

    mov cl, byte [rdi]
    cmp cl, ')'
    je .eval

    dec al
    jz .state.comma

    cmp cl, '0'
    jb .state.comma
    cmp cl, '9'
    ja .state.comma

    mov byte [rdx], cl
    inc rdx

    inc rdi
    jmp .second_number.next_digit

    .eval:
    ; check if have seen at least one digit
    cmp al, 4
    je .state.comma

    lea rdi, [rel number0]
    mov esi, 4
    call atou

    mov r14, rax

    lea rdi, [rel number1]
    mov esi, 4
    call atou

    mul r14
    add r13, rax

    jmp .state.comma

    .result:
    mov rdi, r13
    call print_number

    cmp rax, 0
    jl .done
    xor rax, rax
    .done:
    mov rdi, rax
    call exit

section .bss
number0: resd 1
number1: resd 1
buffer: resb BUF_LEN

