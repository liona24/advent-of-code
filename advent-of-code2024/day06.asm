[bits 64]

%include "lib.asm"

%define N_MAX 0x1000
%define N_SHIFT 12


global _start
_start:
    lea rbp, [rel grid]
    xor r12d, r12d  ; height
    xor r13d, r13d  ; width

    .read_grid:
    mov rdi, r12
    shl rdi, N_SHIFT
    add rdi, rbp
    mov rsi, N_MAX
    call read_line

    cmp rax, 0
    jl .done
    je .solve

    or r13d, eax
    cmp r13d, eax
    jne .done

    inc r12d
    cmp r12d, N_MAX
    jae .done
    jmp .read_grid

    .solve:

    mov al, '^'
    xor edx, edx ; y

    .find_start:
    mov edi, edx
    shl edi, N_SHIFT
    add rdi, rbp

    mov ecx, r13d
    repne scasb
    jz .found_start
    inc edx
    jmp .find_start

    .found_start:
    dec rdi
    ; x
    mov ecx, edi
    and ecx, (N_MAX - 1)

    ; result
    mov r15d, 1
    xor byte [rdi], al

    ; y in edx
    ; x in ecx

    ; direction x
    xor ebx, ebx
    ; direction y
    mov r14d, -1

    .next_move:
    add ecx, ebx
    js .result
    cmp ecx, r13d
    jge .result

    add edx, r14d
    js .result
    cmp edx, r12d
    jge .result

    .before_next_move:
    mov edi, edx
    shl edi, N_SHIFT
    add rdi, rbp

    mov al, byte [rdi + rcx]
    cmp al, '#'
    je .change_direction
    jl .next_move

    xor byte [rdi + rcx], al
    inc r15
    jmp .next_move

    .change_direction:
    sub ecx, ebx
    sub edx, r14d

    xchg ebx, r14d
    neg ebx
    jmp .next_move

    .result:
    mov edi, r15d
    call print_number

    cmp rax, 0
    jl .done
    xor rax, rax
    .done:
    mov rdi, rax
    call exit

section .bss
grid: resb (N_MAX*N_MAX)

