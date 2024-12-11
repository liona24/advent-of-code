[bits 64]

%include "lib.asm"

%define LINE_BUFFER_SIZE 0x1000

; %define N 25
%define N 75

; 12 seems to be enough for the challenge. Smaller numbers decrease overhead
; thus improve runtime. Larger hash tables are required for larger inputs.
%define HASH_TABLE_BITS 12
%define HASH_TABLE_SIZE (1 << HASH_TABLE_BITS)
%define HASH_TABLE_MASK 0x8000000000000000
%define GOLDEN_RATIO_64 0x61C8864680B583EB

; rdi: hash table pointer
; rsi: key to hash
; r8: key to probe
; only clobbers rsi, rax, rdx
__hash_table_probe:
    push rbp
    mov rax, GOLDEN_RATIO_64
    xor rsi, rax
    mul rsi
    shr rax, (64 - HASH_TABLE_BITS)

    mov ebp, eax

    .find_slot:
    lea rdx, [rax * 2]
    mov rsi, qword [rdi + 8 * rdx]
    cmp rsi, r8
    je .found_slot
    inc eax
    and eax, (HASH_TABLE_SIZE - 1)
    cmp ebp, eax
    je .error
    jmp .find_slot

    .found_slot:
    mov rax, rdx

    .exit:
    pop rbp
    ret

    .error:
    mov rax, -1
    jmp .exit


; rdi: hash table pointer
; rsi: key to insert
; rdx: value
counters_add:
    push rdx
    mov rax, -1

    mov rcx, HASH_TABLE_MASK
    or rcx, rsi
    cmp rcx, rsi
    je .err

    push rsi
    mov r8, rcx
    call __hash_table_probe
    pop rsi

    cmp rax, 0
    jge .update

    xor r8d, r8d ; find empty slot
    call __hash_table_probe
    cmp rax, 0
    jl .err

    ; store key
    mov qword [rdi + 8 * rax], rcx
    ; zero counters
    mov qword [rdi + 8 * rax + 8], r8

    .update:
    mov rdx, [rsp]
    add [rdi + 8 * rax + 8], rdx

    xor eax, eax
    add rsp, 8
    ret

    .err:
    ; abort on error to catch wrong result
    db 0xCC


global _start
_start:
    lea rdi, [rel line_buffer]
    mov esi, LINE_BUFFER_SIZE
    call read_line

    cmp rax, 0
    jle .exit

    xor r12d, r12d

    lea rdi, [rel line_buffer]
    mov esi, eax

    .next_number:
    call atou

    push rax
    inc r12d

    test esi, esi
    jnz .next_number

    lea rdi, [rel counters]

    .insert_number:
    pop rsi
    mov edx, 1
    call counters_add

    sub r12d, 1
    jnz .insert_number

    .solve:

    ; iteration count
    xor r15d, r15d
    ; current counters
    mov rbp, rdi
    ; next counters
    lea r14, [rel counters_next]

    .next_iter:
    ; i
    xor r12d, r12d
    .next_blink:

    lea rax, [r12 * 2]
    ; key
    mov rsi, [rbp + 8 * rax]
    mov rdx, (~HASH_TABLE_MASK)
    test rsi, rsi
    jz .next_blink.continue
    and rsi, rdx
    ; count
    mov rdx, [rbp + 8 * rax + 8]

    .rule_0:
    test rsi, rsi
    jnz .rule_1

    ; 0 becomes 1
    inc rsi

    jmp .rules_end
    .rule_1:

    mov r13, rdx
    mov rbx, rsi

    lea rdi, [rel line_buffer]
    call utoa

    ; even number of digits?
    test eax, 1
    jnz .rule_2

    ; split digits and create two numbers
    mov ebx, eax

    mov esi, ebx
    shr esi, 1
    lea rdi, [rel line_buffer]

    call atou

    mov rdi, r14
    mov rsi, rax
    mov rdx, r13
    call counters_add

    mov esi, ebx
    shr esi, 1
    lea rdi, [rel line_buffer]
    add rdi, rsi

    call atou
    mov rsi, rax
    mov rdx, r13

    jmp .rules_end

    .rule_2:
    ; multiply by 2024
    mov rax, 2024
    mul rbx
    mov rsi, rax

    mov rdx, r13
    .rules_end:

    mov rdi, r14
    call counters_add

    .next_blink.continue:
    inc r12d
    cmp r12d, HASH_TABLE_SIZE
    jne .next_blink

    xchg rbp, r14

    ; clear next counters
    mov ecx, (HASH_TABLE_SIZE*2)
    mov rdi, r14
    xor eax, eax
    rep stosq

    inc r15d
    cmp r15d, N
    jne .next_iter

    ; aggreate
    xor r15d, r15d
    ; i
    xor r12d, r12d
    .next_aggregate:

    lea rax, [r12 * 2]
    mov rax, [rbp + 8 * rax + 8]
    add r15, rax

    inc r12d
    cmp r12d, HASH_TABLE_SIZE
    jne .next_aggregate

    mov rdi, r15
    call print_number

    .exit:
    mov edi, eax
    call exit

section .bss
line_buffer: resb LINE_BUFFER_SIZE
counters:
    resq HASH_TABLE_SIZE ; keys
    resq HASH_TABLE_SIZE ; + values
counters_next:
    resq HASH_TABLE_SIZE ; keys
    resq HASH_TABLE_SIZE ; + values

