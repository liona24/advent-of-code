[bits 64]

section .text

exit:
    mov rax, 60
    syscall
    db 0xCC

; void* memcpy(void* dst, const void* src, u64 len)
; rdi - dst buffer
; rsi - src buffer
; rdx - length to copy
; -> return the dst buffer
memcpy:
    mov rcx, rdx
    mov rax, rdi
    rep movsb
    ret

; u64 read_line(const void* buffer, u64 len)
; read a line from stdin. strip the trailing new line.
; rdi - buffer
; rsi - len
; -> return number of bytes read
read_line:
    ; total count
    xor r8, r8

    ; output len
    mov r9, rsi
    ; output buf
    mov rsi, rdi

    xor rdi, rdi    ; stdin
    mov rdx, 1      ; one char at a time

    .next_char:
    xor rax, rax    ; read
    syscall

    cmp rax, 0
    jl .error
    je .done

    ; check for new line
    cmp byte [rsi], 0x0a
    je .done

    ; increment length
    inc r8
    inc rsi

    ; check if we have more space
    cmp r9, r8
    ja .next_char

    .done:
    mov rax, r8
    ret

    .error:
    mov rdi, 1
    call exit
    db 0xCC

; void print(const void* buffer, u64 len)
; write buffer to stdout
; rdi - buffer
; rsi - len
print:
    mov rdx, rsi
    mov rsi, rdi

    mov rax, 1
    mov rdi, rax
    syscall

    ret


; u64 atou(const char* str, u64 len)
; convert string argument to number with radix 10
; rdi - string
; rsi - string length
; return the converted number, returns zero for invalid numbers. :)
atou:
    ; find the last digit
    xor rcx, rcx

    .find_non_digit:
    cmp rcx, rsi
    jae .convert

    mov al, byte [rdi + rcx]
    cmp al, '0'
    jb .convert
    cmp al, '9'
    ja .convert
    inc rcx
    jmp .find_non_digit

    .convert:
    ; base 10, current power
    mov rax, 1
    ; accumulator
    xor rsi, rsi

    .convert_next:
    test rcx, rcx
    jz .done

    dec rcx

    ; read digit
    movzx rdx, byte[rdi + rcx]
    sub rdx, '0'

    ; accumulate
    mov r8, rax
    mul rdx
    add rsi, rax

    ; next power
    mov rax, 10
    mul r8

    jmp .convert_next

    .done:
    mov rax, rsi
    ret

; u64 utoa(u64 number, char* str, u64 len)
; convert number to string in argument with radix 10
; rdi - number to convert
; rsi - string to place result into
; rdx - maximum length of result string
; return the actual string length
utoa:
    ; preserve actual result string
    push rsi
    push rdx

    ; temporary buffer to assemble the string
    ; we assemble from the back to the front
    sub rsp, 24
    mov rsi, rsp

    mov rax, rdi
    ; preserve rdi for later copy
    mov rdi, rsi

    ; index
    xor rcx, rcx
    ; modulo
    mov r8, 10

    .collect_bases:
    xor rdx, rdx
    div r8

    ; remainder in rdx
    add dl, '0'
    mov byte [rsi + rcx], dl
    inc rcx

    test rax, rax
    jz .done
    jmp .collect_bases

    .done:
    mov rax, rcx
    dec rcx

    add rsp, 24
    pop rdx
    pop rsi

    cmp rcx, rdx
    jae .error

    .copy:
    mov r8b, byte [rdi + rcx]
    mov byte [rsi], r8b

    inc rsi
    dec rcx
    jns .copy

    ret

    .error:
    xor rax, rax
    dec rax
    ret
