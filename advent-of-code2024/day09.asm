[bits 64]

%include "lib.asm"

%define MAX_LEN 0x8000

; calculate checksum for x consecutive blocks with given id at index n0
; rdi: n0
; rsi: id
; rax: x
; only clobbers rax, rdx
checksum_n:
    ; 2nx + x^2 - x
    lea rdx, [rax + 2 * rdi - 1]
    mul rdx

    ; / 2
    shr rax, 1
    mul rsi

    ret


global _start
_start:
    lea rbp, [rel line]
    mov rdi, rbp
    mov esi, MAX_LEN
    call read_line

    cmp rax, 0
    jle .error

    mov r15d, eax

    ; forward increment
    xor r12d, r12d

    ; backward increment
    mov r13d, eax
    sub r13d, 1

    ; result
    xor r11, r11
    ; actual size
    xor r10, r10

    .next_pair:
    cmp r12d, r13d
    jg .done

    ; n0
    mov rdi, r10

    ; rax full slots
    movzx rax, byte [rbp + r12]
    sub al, '0'

    ; advance file position
    add r10, rax

    ; id of this slot
    mov esi, r12d
    shr esi, 1

    call checksum_n
    add r11, rax

    .fill_free_slot:
    cmp r12d, r13d
    jge .done

    ; rax free slots
    movzx rax, byte [rbp + r12 + 1]
    sub al, '0'

    ; filler block
    movzx r8, byte [rbp + r13]
    sub r8b, '0'

    ; n0
    mov rdi, r10

    ; id
    mov esi, r13d
    shr esi, 1

    cmp r8b, al
    jg .fill_partial

    ; use full filler block
    sub byte [rbp + r12 + 1], r8b
    sub r13d, 2

    add r10, r8

    ; x
    mov eax, r8d

    call checksum_n
    add r11, rax

    jmp .fill_free_slot

    .fill_partial:
    sub byte [rbp + r13], al
    add r12d, 2

    add r10, rax

    call checksum_n
    add r11, rax

    jmp .next_pair

    .done:
    mov rdi, r11
    call print_number

    .exit:
    xor eax, eax
    .error:
    mov rdi, rax
    call exit

section .bss
line: resb MAX_LEN
