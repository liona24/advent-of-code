[bits 64]

%include "lib.asm"

%define LINE_BUF_LEN 0x1000

%define SECONDS 100
%define WIDTH 101
%define HEIGHT 103
; %define WIDTH 11
; %define HEIGHT 7

%define P_X (12)
%define P_Y (8)
%define V_X (4)
%define V_Y (0)

global _start
_start:
    ; space for coordinates and velocity
    sub rsp, 16
    lea rbp, [rel line_buf]
    ; result
    lea r15, [rel quadrants]

    .read_next:
    mov rdi, rbp
    mov esi, LINE_BUF_LEN
    call read_line

    cmp eax, 0
    jl .exit
    je .done

    mov esi, eax
    mov rdi, rbp

    call atoi
    mov dword [rsp + P_X], eax
    call atoi
    mov dword [rsp + P_Y], eax
    call atoi
    mov dword [rsp + V_X], eax
    call atoi
    mov dword [rsp + V_Y], eax

    mov ebx, SECONDS
    ; quadrant coordinates
    xor r14d, r14d

    ; V_Y
    mov esi, HEIGHT
    imul rbx
    movsx rdx, dword [rsp + P_Y]
    add rax, rdx
    mov rdx, rax
    sar rdx, 63
    idiv rsi

    cmp rdx, 0
    jge .classify_y
    add rdx, HEIGHT

    .classify_y:
    sub edx, (HEIGHT/2)
    jz .read_next ; on border, discard

    ; this sets r14 to 2 for the left quadrants and to 0 for the right quadrants
    sar edx, 31
    neg edx
    add r14d, edx
    shl r14d, 1

    mov esi, WIDTH
    movsx rax, dword [rsp + V_X]
    imul rbx
    movsx rdx, dword [rsp + P_X]
    add rax, rdx
    mov rdx, rax
    sar rdx, 63
    idiv rsi

    cmp rdx, 0
    jge .classify_x
    add rdx, WIDTH

    .classify_x:
    sub edx, (WIDTH/2)
    jz .read_next ; on border, discard

    sar edx, 31
    neg edx
    add r14d, edx

    .inc_quadrant:
    inc dword [r15 + r14 * 4]
    jmp .read_next

    .done:
    mov eax, dword [r15 + 0]
    mul dword [r15 + 4]
    mul dword [r15 + 8]
    mul dword [r15 + 12]
    mov edi, eax
    call print_number

    .exit:
    mov edi, eax
    call exit

section .bss
line_buf: resb LINE_BUF_LEN
quadrants: resd 4
