[bits 64]

%include "lib.asm"

%define LINE_BUF_LEN 0x100

op_adv:
    mov ecx, edi
    sar r12, cl
    ret
op_bxl:
    xor r13, rsi
    ret
op_bst:
    mov r13, rdi
    and r13, 7
    ret
op_jnz:
    test r12, r12
    jz .end
    lea rbp, [rel line_buffer]
    add rbp, rsi
    .end:
    ret
op_bxc:
    xor r13, r14
    ret
op_out:
    and edi, 7
    add edi, '0'
    mov ecx, `\0,`
    or ecx, edi
    push rcx

    ; 1 = write
    mov rax, 1
    ; fd = 1 = STDOUT
    mov rdi, rax
    ; pointer
    mov rsi, rsp
    ; count
    mov rdx, 2
    syscall

    add rsp, 8
    ret
op_bdv:
    mov r13, r12
    mov ecx, edi
    sar r13, cl
    ret
op_cdv:
    mov r14, r12
    mov ecx, edi
    sar r14, cl
    ret

global _start
_start:
    lea rbp, [rel line_buffer]

    mov rdi, rbp
    mov esi, LINE_BUF_LEN
    call read_line
    cmp rax, 0
    jl .exit

    mov rdi, rbp
    mov esi, eax
    call atou

    ; A
    mov r12, rax

    mov rdi, rbp
    mov esi, LINE_BUF_LEN
    call read_line
    cmp rax, 0
    jl .exit

    mov rdi, rbp
    mov esi, eax
    call atou

    ; B
    mov r13, rax

    mov rdi, rbp
    mov esi, LINE_BUF_LEN
    call read_line
    cmp rax, 0
    jl .exit

    mov rdi, rbp
    mov esi, eax
    call atou

    ; C
    mov r14, rax

    ; skip new line
    mov esi, 1
    call read_line
    test rax, rax
    jne .exit

    mov rdi, rbp
    mov esi, LINE_BUF_LEN
    call read_line
    cmp rax, 0
    jl .exit

    xor r15d, r15d
    mov rdi, rbp
    mov rsi, rax
    .parse_insns:
    call atou
    and al, 7
    mov byte [rbp + r15], al

    add r15, 1
    test rsi, rsi
    jnz .parse_insns

    lea r15, [rbp + r15]

    .loop:

    cmp rbp, r15
    jae .done

    ; opcode
    movzx rax, byte [rbp]
    lea rcx, [rel opcodes]
    mov rcx, qword [rcx + rax * 8]

    ; operand
    movzx rax, byte [rbp + 1]
    add rbp, 2

    ; combo in rdi, literal in rsi
    mov rdi, rax
    mov rsi, rax
    cmp al, 4
    cmove rdi, r12
    cmp al, 5
    cmove rdi, r13
    cmp al, 6
    cmove rdi, r14

    call rcx
    jmp .loop

    .done:
    xor eax, eax
    .exit:
    mov edi, eax
    call exit

section .rodata
opcodes: 
    dq op_adv
    dq op_bxl
    dq op_bst
    dq op_jnz
    dq op_bxc
    dq op_out
    dq op_bdv
    dq op_cdv

section .bss
line_buffer: resb LINE_BUF_LEN
