[bits 64]

%include "lib.asm"

%define BUF_LEN 0x8000
%define LINE_BUF_LEN 0x1000
%define HASH_TABLE_SIZE 0x4000
%define HASH_TABLE_BITS 14
%define HASH_TABLE_MASK 0x8000000000000000
%define GOLDEN_RATIO_64 0x61C8864680B583EB

; rdi: hash table pointer
; rsi: key to hash
; r8: key to probe
; only clobbers rdi, rsi, rax, rdx
__hash_table_probe:
    push rbp
    mov rax, GOLDEN_RATIO_64
    mul rsi
    shr rax, (64 - HASH_TABLE_BITS)

    mov ebp, eax

    .find_slot:
    mov rsi, qword [rdi + 8 * rax]
    cmp rsi, r8
    je .found_slot
    inc eax
    and eax, (HASH_TABLE_SIZE - 1)
    cmp ebp, eax
    je .error
    jmp .find_slot
    .error:
    mov rax, -1
    .found_slot:
    pop rbp
    ret


; rdi: hash table pointer
; rsi: key to insert
hash_table_insert:
    mov rcx, HASH_TABLE_MASK
    or rcx, rsi
    cmp rcx, rsi
    je .err

    xor r8d, r8d ; find empty slot
    call __hash_table_probe
    cmp rax, 0
    jl .err

    mov qword [rdi + 8 * rax], rcx

    xor rax, rax
    .err:
    ret


; rdi: hash table pointer
; rsi: key to lookup
hash_table_lookup:
    mov r8, HASH_TABLE_MASK
    or r8, rsi
    jmp __hash_table_probe


global _start
_start:
    lea r15, [rel page_numbers]

    .read_page_ordering_rule:
    lea rbp, [rel line_buffer]
    mov rdi, rbp
    mov rsi, LINE_BUF_LEN
    call read_line

    cmp rax, 0
    jl .done
    je .process_page_numbers

    mov rsi, rax
    mov rdi, rbp
    call atou
    mov ebx, eax
    call atou

    lea rdi, [rel hashtable_1]
    mov esi, ebx
    shl rsi, 32
    or rsi, rax
    call hash_table_insert

    jmp .read_page_ordering_rule

    .process_page_numbers:
    lea rbp, [rel line_buffer]
    mov rdi, rbp
    mov rsi, LINE_BUF_LEN
    call read_line

    cmp rax, 0
    jle .result

    xor r14d, r14d
    mov rdi, rbp
    mov rsi, rax

    .parse_number:
    call atou
    mov dword [r15 + r14 * 4], eax
    inc r14
    test rsi, rsi
    jnz .parse_number

    ; i
    xor r12d, r12d
    ; prev value
    mov ebp, dword [r15]

    .forward:
    inc r12d
    cmp r12d, r14d
    je .forward.done

    mov r13d, r12d

    .forward.inner:
    ; check violation
    mov esi, dword [r15 + r13 * 4]
    shl rsi, 32
    or rsi, rbp
    lea rdi, [rel hashtable_1]

    call hash_table_lookup
    cmp eax, 0
    jge .invalid

    inc r13d
    cmp r13d, r14d
    jb .forward.inner

    mov ebp, dword [r15 + r12 * 4]
    jmp .forward

    .forward.done:
    shr r14d, 1
    mov eax, dword [r15 + r14 * 4]
    lea rdi, [rel result]
    add [rdi], rax

    .invalid:
    jmp .process_page_numbers

    .result:
    lea rdi, [rel result]
    mov rdi, qword [rdi]
    call print_number

    cmp rax, 0
    jl .done
    xor rax, rax
    .done:
    mov rdi, rax
    call exit

section .bss
result: resq 1
line_buffer: resb LINE_BUF_LEN
page_numbers: resd LINE_BUF_LEN
hashtable_1: resq HASH_TABLE_SIZE

