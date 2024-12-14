[bits 64]

%define LINE_BUF_LEN 0x1000

%define A_X (0*8)
%define A_Y (0*8+4)
%define B_X (1*8)
%define B_Y (1*8+4)
%define P_X (2*8)
%define P_Y (2*8+4)

%include "lib.asm"

global _start
_start:
    ; final solution
    xor r14, r14
    lea r15, [rel line_buf]

    lea rbp, [rel buttons]
    xor ebx, ebx

    .parse_one:
    mov rdi, r15
    mov esi, LINE_BUF_LEN
    call read_line

    cmp rax, 0
    jl .exit
    je .print_solution

    mov rdi, r15
    mov esi, eax
    call atou
    mov dword [rbp + rbx * (4+4)], eax

    call atou
    mov dword [rbp + rbx * (4+4) + 4], eax

    inc ebx
    cmp ebx, 3
    jne .parse_one

    xor ebx, ebx

    ; TODO: change between day 2 and day 1:
    mov rax, 10000000000000
    ; xor rax, rax

    ; X
    mov r12d, dword [rbp + P_X]
    add r12, rax
    ; Y
    mov r13d, dword [rbp + P_Y]
    add r13, rax

    ; determinant
    mov eax, dword [rbp + A_X]
    imul dword [rbp + B_Y]
    mov edi, eax
    mov eax, dword [rbp + A_Y]
    imul dword [rbp + B_X]

    .determinant:
    sub edi, eax
    jz .determinant_zero

    ; inverse
    ; A_X  B_X
    ; A_Y  B_Y
    ; =>
    ; B_Y -B_X
    ;-A_Y  A_X
    mov eax, dword [rbp + A_X]
    xchg eax, dword [rbp + B_Y]
    mov dword [rbp + A_X], eax
    neg dword [rbp + A_Y]
    neg dword [rbp + B_X]

    movsx rdi, edi

    ; check if solvable with integers
    movsx rax, dword [rbp + A_X]
    imul r12
    mov rsi, rax
    movsx rax, dword [rbp + B_X]
    imul r13
    add rax, rsi

    mov rdx, rax
    sar rdx, 63
    idiv rdi
    test rdx, rdx
    jnz .not_valid

    ; nA
    mov r8, rax

    movsx rax, dword [rbp + A_Y]
    imul r12
    mov rsi, rax
    movsx rax, dword [rbp + B_Y]
    imul r13
    add rax, rsi

    mov rdx, rax
    sar rdx, 63
    idiv rdi
    test rdx, rdx
    jnz .not_valid

    ; found a solution
    lea rsi, [rax + 2 * r8]
    add rsi, r8
    add r14, rsi

    jmp .not_valid

    .determinant_zero:
    ; interestingly, this case is not present in the input. I _think_ it works
    ; correctly...

    ; okay there might be a solution
    ; just press B as often as possible and check if 
    ; leftover can be done with A?
    ; note that matrix is not yet inversed
    mov rax, r12
    mov rdx, rax
    sar rdx, 63
    movsx rsi, dword [rbp + B_X]
    idiv rsi

    ; how often we pressed B
    mov rdi, rax

    mov rax, rdx
    sar rdx, 63
    movsx rsi, dword [rbp + A_X]
    idiv rsi

    test rdx, rdx
    jne .not_valid

    ; score
    lea r8, [rdi + 2 * rax]
    add r8, rax

    ; check if Y checks out
    movsx rsi, dword [rbp + A_Y]
    imul rsi
    sub rax, r13
    neg rax

    xchg rdi, rax
    movsx rsi, dword [rbp + B_Y]
    imul rsi
    cmp rdi, rax
    jne .not_valid

    add r14, r8

    .not_valid:
    xor ebx, ebx

    ; skip new line
    mov rdi, r15
    mov esi, LINE_BUF_LEN
    call read_line

    cmp rax, 0
    jne .exit
    jmp .parse_one

    .print_solution:

    mov rdi, r14
    call print_number

    .exit:
    mov edi, eax
    call exit

section .bss
line_buf: resb LINE_BUF_LEN
buttons: resd (3*2) 
