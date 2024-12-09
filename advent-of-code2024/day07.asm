[bits 64]

%include "lib.asm"

%define LINE_BUFFER_LEN 0x1000


global solve
solve:
    ; rdi: expected result
    ; rsi: numbers pointer
    ; rdx: number of numbers

    push rbp
    push r14
    push r15

    .tail:
    xor eax, eax
    test rdx, rdx
    jz .maybe_good

    dec rdx
    mov r15, qword [rsi + 8 * rdx]
    mov rbp, rdi
    mov r14, rdx

    mov rax, rbp
    xor rdx, rdx
    div r15

    test rdx, rdx
    jnz .skip

    mov rdi, rax
    mov rdx, r14
    call solve

    test eax, eax
    jz .good

    mov rdi, rbp

    .skip:
    sub rdi, r15
    jl .not_good

    mov rdx, r14
    jmp .tail

    .maybe_good:
    test rdi, rdi
    jz .good

    .not_good:
    mov eax, 1

    .good:
    pop r15
    pop r14
    pop rbp
    ret 


global _start
_start:
    lea rbp, [rel line_buffer]
    xor r15d, r15d

    .next_line:
    mov ecx, (LINE_BUFFER_LEN / 8)
    mov rdi, rbp
    xor eax, eax
    rep stosq

    mov rdi, rbp
    mov esi, LINE_BUFFER_LEN
    call read_line

    cmp rax, 0
    jl .exit
    jz .result

    lea r12, [rel numbers]
    xor r13d, r13d

    mov rdi, rbp
    mov rsi, rax

    .next_number:
    call atou
    mov qword [r12 + r13 * 8], rax
    add r13d, 1
    test esi, esi
    jz .solve
    jmp .next_number

    .solve:
    dec r13d
    lea rsi, [r12 + 8]
    mov rdi, qword [r12]
    push rdi
    mov edx, r13d
    call solve

    test eax, eax
    jnz .next_line

    add r15, qword [r12]
    jmp .next_line

    .result:
    mov rdi, r15
    call print_number
    cmp eax, 0
    jl .exit
    xor eax, eax
    .exit:
    mov rdi, rax
    call exit

section .bss
line_buffer: resb LINE_BUFFER_LEN
numbers: resq (0x1000 / 8)
