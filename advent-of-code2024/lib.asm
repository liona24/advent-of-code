[bits 64]

; convert string to number base 10 ignoring invalid characters
; [in]  rdi: char*  - input string
; [in]  rsi: size_t - input string length
; [out] rax: size_t - string converted to unsigned integer base 10
;                      rdi, rsi can be assumed to be consistent with the last
;                      character read
atou:
    xor rax, rax
    mov r8, 10
    ; keep track of converted characters to only skip at the beginning
    xor r9b, r9b

    .next_char:
    cmp rsi, 0
    je .done

    mov dl, byte [rdi]
    cmp dl, '0'
    jb .skip

    sub dl, '0'
    movzx rcx, dl

    cmp rdx, r8
    jae .skip

    or r9b, 1

    mul r8
    add rax, rcx

    .continue:
    inc rdi
    dec rsi
    jmp .next_char

    .skip:
    test rax, rax
    jnz .done

    test r9b, r9b
    jnz .done

    jmp .continue

    .done:
    ret


global print_number
; [in] rdi: ssize_t - number to print
print_number:
    lea rsi, [rsp - 1]

    mov al, 0xa  ; newline
    mov byte [rsi], al
    dec rsi

    xor r8, r8

    cmp rdi, 0
    jge .convert_to_string

    neg rdi
    inc r8

    .convert_to_string:

    xor rdx, rdx
    mov rax, rdi
    mov rdi, 10
    div rdi

    add dl, '0'
    mov byte [rsi], dl
    dec rsi

    mov rdi, rax
    test rax, rax
    jne .convert_to_string

    test r8, r8
    jz .no_sign

    mov al, '-'
    mov byte [rsi], al
    dec rsi

    .no_sign:
    inc rsi

    ; 1 = write
    mov rax, 1
    ; fd = 1 = STDOUT
    mov rdi, rax
    ; count
    mov rdx, rsp
    sub rdx, rsi
    syscall

    ret



global read_line
; read a line from stdin
; [in,out] rdi: char*   - buffer to fill
; [in]     rsi: size_t  - input buffer length
; [out]    rax: ssize_t - length of line read (excluding newline)
;                          or -1 on error
read_line:
    mov r8, rsi
    mov rsi, rdi
    xor r9, r9

    .next_char:
    cmp r8, r9
    je .done
    ; 0 = read
    xor rax, rax
    ; fd = 0 = STDIN
    xor rdi, rdi
    ; count = 1
    mov rdx, 1
    syscall

    cmp rax, 0
    jl .error
    je .done

    mov dil, byte [rsi]
    cmp dil, 0x0a ; newline
    je .done

    inc rsi
    inc r9
    jmp .next_char

    .done:
    mov rax, r9
    .error:
    ret


; calculate absolute value of %1 using temporary register %2 into %1
%macro abs 2
    mov %2, %1
    sar %2, 63
    add %1, %2
    xor %1, %2
%endmacro


global exit
exit:
    ; 60 = exit
    xor rdi, rdi
    mov rax, 0x3c
    syscall
    db 0xCC

