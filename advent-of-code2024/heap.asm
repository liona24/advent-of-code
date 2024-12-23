
global __heap_push
__heap_push:
    ; rdi heap array pointer
    ; rsi heap len
    ; rdx element to insert
    .up:
    test rsi, rsi
    jz .done

    mov r8, rsi
    add r8, 1
    shr r8, 1
    sub r8, 1
    mov rax, qword [rdi + 8 * r8]
    cmp rax, rdx
    jle .done

    mov qword [rdi + 8 * r8], rdx
    mov qword [rdi + 8 * rsi], rax
    mov rsi, r8
    jmp .up

    .done:
    mov qword [rdi + 8 * rsi], rdx
    ret


global __heap_pop
__heap_pop:
    ; rdi heap array pointer
    ; rsi heap len
    push qword [rdi]

    sub rsi, 1
    ; element that trickles down
    mov r10, qword [rdi + 8 * rsi]
    ; current index
    xor rdx, rdx
    ; index to swap with
    xor r9, r9

    .down:
    ; child index
    mov r8, rdx
    ; current lowest child
    mov rax, r10

    ; left child
    shl r8, 1
    add r8, 1

    cmp r8, rsi
    jae .done

    mov rcx, qword [rdi + 8 * r8]
    cmp rax, rcx
    jle .skip_left

    mov r9, r8
    mov rax, rcx

    .skip_left:

    ; right child
    add r8, 1
    cmp r8, rsi
    jae .skip_right

    mov rcx, qword [rdi + 8 * r8]
    cmp rax, rcx
    jle .skip_right

    mov r9, r8
    mov rax, rcx

    .skip_right:
    cmp r9, rdx
    je .done

    ; actually swap the smallest child up
    mov qword [rdi + 8 * rdx], rax
    mov rdx, r9
    jmp .down

    .done:
    ; store the element at its final destination
    mov qword [rdi + rdx * 8], rax

    pop rax
    ret

