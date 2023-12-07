[bits 64]

; %define PROBLEM 1
%define PROBLEM 2

%define NUM_CARDS 5

%if NUM_CARDS > 8 || NUM_CARDS <= 0
%error "NUM_CARDS need be 0 < NUM_CARDS <= 8"
%endif

%macro trap 0
    db 0xCC
%endmacro

struc hand
    .cards:
        resb NUM_CARDS
        alignb 8
    .runs:
        resb NUM_CARDS
        alignb 8
    .bids:
        resq 1
endstruc

section .bss
%define MAX_HANDS 1024

global hands
hands:
    resb (hand_size * MAX_HANDS)

next_hand:
    resq 1

section .text

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

; int compare_u8(const u8* a, const u8* b)
; rdi - u8 pointer a
; rsi - u8 pointer b
compare_u8:
    xor rax, rax
    mov dil, byte [rdi]
    mov sil, byte [rsi]

    cmp dil, sil
    ja .greater
    jb .less
    jmp .return

    .greater:
    inc rax
    jmp .return

    .less:
    dec rax

    .return:
    ret

; int compare_u64(const u64* a, const u64* b)
; rdi - u64 pointer a
; rsi - u64 pointer b
compare_u64:
    xor rax, rax
    mov rdi, qword [rdi]
    mov rsi, qword [rsi]

    cmp rdi, rsi
    ja .greater
    jb .less
    jmp .return

    .greater:
    inc rax
    jmp .return

    .less:
    dec rax

    .return:
    ret

; void insertion_sort(void* array, u64 nmemb, u64 size, int (*compare)(const void*, const void*))
; sort the given array with `nmemb` elements of size `size` each according to the given compare function
; rdi - array
; rsi - nmemb
; rdx - size
; rcx - compare function
insertion_sort:
    push r12
    push r14
    push r15
    push rbx
    push rbp
    mov rbp, rsp

    ; short circuit, we need at least one element
    test rsi, rsi
    jz .done

    ; [rbp - 8] - array
    push rdi
    ; [rbp - 16] - pointer to the outer slot
    push rdi
    ; [rbp - 24] - pointer to the inner slot
    push rdi
    ; temporary slot
    sub rsp, rdx
    mov r15, rsp

    ; compare function in rbx
    mov rbx, rcx
    ; preserve element size
    mov r14, rdx

    ; outer loop size
    mov r12, rsi
    .outer_loop:
        dec r12
        jz .done

        ; fetch pointer to current object
        mov rax, qword [rbp - 16]
        add rax, r14
        mov qword [rbp - 16], rax

        ; initialize inner loop
        mov rax, qword [rbp - 8]
        mov qword [rbp - 24], rax

        .inner_loop:
            ; left hand side, current outer slot
            mov rdi, qword [rbp - 16]
            ; right hand side
            mov rsi, qword [rbp - 24]

            cmp rdi, rsi
            je .outer_loop

            call rbx

            cmp eax, 0
            jge .tmp_is_equal_or_greater

            ; current outer slot is smaller. swap elements

            ; memcpy array[inner i] to tmp slot
            mov rdi, r15
            mov rsi, qword [rbp - 24]
            mov rdx, r14
            call memcpy

            ; memcpy array[outer i] to array[inner i]
            mov rdi, qword [rbp - 24]
            mov rsi, qword [rbp - 16]
            mov rdx, r14
            call memcpy

            ; memcpy tmp to array[outer i]
            mov rdi, qword [rbp - 16]
            mov rsi, r15
            mov rdx, r14
            call memcpy

            .tmp_is_equal_or_greater:

            add qword [rbp - 24], r14
            jmp .inner_loop
        jmp .outer_loop

    .done:
    mov rsp, rbp
    pop rbp
    pop rbx
    pop r15
    pop r14
    pop r12
    ret


; struct hand* parse_hand(const char* str, u64 len)
; parse the hand given in the string argument into proper hand structure
; rdi - string to parse
; rsi - string length
; -> return the parsed hand pointer
hand_parse:
    push rbx

    ; check for the first 5 characters + space
    mov rax, rsi
    cmp rax, 6
    jle .error

    ; allocate a new hand
    lea rdx, [rel next_hand]
    mov rax, qword [rdx]

    ; adjust end pointer
    mov rbx, rax
    add rbx, hand_size
    mov qword [rdx], rbx

    ; check if we have space
    cmp rax, (MAX_HANDS * hand_size)
    jae .error

    ; load hand address into rax
    lea rbx, [hands + rax]

    xor rcx, rcx

    .parse_cards:
    ; load one card
    mov r8b, byte [rdi + rcx]

    %macro case 2
        cmp r8b, %1
        jne %%next
        mov r8b, %2
        jmp .endcase
        %%next:
    %endmacro

    ; convert it
    case '2', 2
    case '3', 3
    case '4', 4
    case '5', 5
    case '6', 6
    case '7', 7
    case '8', 8
    case '9', 9
    case 'T', 10
    case 'J', 11
    case 'Q', 12
    case 'K', 13
    case 'A', 14
    jmp .error

    .endcase:
    mov byte [rbx + hand.cards + rcx], r8b

    inc rcx
    cmp rcx, NUM_CARDS
    jb .parse_cards

    ; skip the space
    inc rcx
    sub rsi, rcx
    add rdi, rcx
    ; convert the bid
    call atou

    mov qword [rbx + hand.bids], rax
    mov rax, rbx
    jmp .ret

    .error:
    xor rax, rax

    .ret:
    pop rbx
    ret

; struct hand* hand_fill_runs1(struct hand* h)
; fill the runs for the given hand (part I)
; rdi - hand pointer
; -> return the hand pointer
hand_fill_runs1:
    push r15
    push rbp
    mov rbp, rsp

    mov r15, rdi

    ; [rbp - 8] - temporary cards array
    xor rax, rax
    push rax

    ; copy cards to temporary location
    lea rsi, [r15 + hand.cards]
    lea rdi, [rbp - 8]
    mov rdx, NUM_CARDS
    call memcpy

    ; sort the cards
    mov rdi, rax
    mov rsi, NUM_CARDS
    mov rdx, 1
    lea rcx, [rel compare_u8]
    call insertion_sort

    ; rcx - i
    xor rcx, rcx
    ; r8 - run length
    xor r8, r8
    ; al - current card
    mov al, byte [rbp - 8]

    .fill_run:
        cmp rcx, 5
        jae .end_loop

        cmp al, byte [rbp - 8 + rcx]
        je .inc_run_length

        mov al, byte [rbp - 8 + rcx]

        mov rdx, hand.runs + NUM_CARDS
        sub rdx, r8
        inc byte [r15 + rdx]
        xor r8, r8

        .inc_run_length:
        inc r8

        inc rcx
        jmp .fill_run

    .end_loop:
    mov rdx, hand.runs + NUM_CARDS
    sub rdx, r8
    inc byte [r15 + rdx]

    mov rax, r15

    mov rsp, rbp
    pop rbp
    pop r15
    ret

; struct hand* hand_fill_runs2(struct hand* h)
; fill the runs for the given hand (part II)
; rdi - hand pointer
; -> return the hand pointer
hand_fill_runs2:
    push r15
    push r14
    push r13
    push rbp
    mov rbp, rsp

    mov r15, rdi

    ; [rbp - 8] - temporary cards array
    xor rax, rax
    push rax

    ; r13 - all the computed runs
    sub rsp, 8 * 13
    mov r13, rsp

    ; zero them out
    mov rcx, 13
    mov rdi, r13
    rep stosq

    ; clear out all jokers to 0
    xor rcx, rcx
    lea rdi, [r15 + hand.cards]
    .clear_jokers:
        cmp byte [rdi + rcx], 11
        jne .continue_clear_jokers
        mov byte [rdi + rcx], 0

        .continue_clear_jokers:
        inc rcx
        cmp rcx, NUM_CARDS
        jb .clear_jokers

    ; current run we compute
    xor r14, r14
    .compute_one_run:
    ; copy cards to temporary location
    lea rsi, [r15 + hand.cards]
    lea rdi, [rbp - 8]
    mov rdx, NUM_CARDS
    call memcpy

    ; replace jokers with current assumption
    mov rdi, rax
    mov rax, r14
    add rax, 2

    xor rcx, rcx
    .replace_jokers:
        cmp byte [rdi + rcx], 0
        jne .continue
        mov byte [rdi + rcx], al

        .continue:
        inc rcx
        cmp rcx, NUM_CARDS
        jb .replace_jokers

    ; sort the cards
    mov rsi, NUM_CARDS
    mov rdx, 1
    lea rcx, [rel compare_u8]
    call insertion_sort

    ; rcx - i
    xor rcx, rcx
    ; r8 - run length
    xor r8, r8
    ; al - current card
    mov al, byte [rbp - 8]
    ; current run
    lea r9, [r13 + r14 * 8]

    .fill_run:
        cmp al, byte [rbp - 8 + rcx]
        je .inc_run_length

        mov al, byte [rbp - 8 + rcx]

        mov rdx, NUM_CARDS
        sub rdx, r8
        inc byte [r9 + rdx]
        xor r8, r8

        .inc_run_length:
        inc r8

        inc rcx
        cmp rcx, NUM_CARDS
        jb .fill_run

    mov rdx, NUM_CARDS
    sub rdx, r8
    inc byte [r9 + rdx]

    inc r14
    cmp r14, 13
    jb .compute_one_run

    ; sort the runs
    mov rdi, r13
    mov rsi, 13
    mov rdx, 8
    lea rcx, [rel hand_compare_run_inv]
    call insertion_sort

    ; take the best one
    lea rdi, [r15 + hand.runs]
    mov rsi, r13
    mov rdx, NUM_CARDS
    call memcpy

    mov rax, r15

    mov rsp, rbp
    pop rbp
    pop r13
    pop r14
    pop r15
    ret


; int hand_compare_run_inv(const u8[NUM_CARDS] run1, const u8[NUM_CARDS] run2)
; return the ordering of the given runs (note that this ordering is inverted):
;   -1 <=> run1 > run2
;    0 <=> run1 == run2
;    1 <=> run1 < run2
; rdi - first run ptr
; rsi - secnd run ptr
; -> return the ordering
hand_compare_run_inv:
    xor rax, rax
    mov rcx, NUM_CARDS
    repe cmpsb
    ja .greater
    jb .less
    ret
    .greater:
    inc rax
    ret
    .less:
    dec rax
    ret


; int hand_compare(const struct hand* h1, const struct hand* h2)
; return the ordering of the given hands:
;   -1 <=> h1 < h2
;    0 <=> h1 == h2
;    1 <=> h1 > h2
; rdi - first hand ptr
; rsi - secnd hand ptr
; -> return the ordering
hand_compare:
    xor rax, rax

    %if 0
    ; alternative implementation
    mov r8, rdi
    mov r9, rsi

    lea rdi, [r8 + hand.runs]
    lea rsi, [r9 + hand.runs]
    mov rcx, NUM_CARDS

    repe cmpsb
    jb .greater
    ja .less

    lea rdi, [r8 + hand.cards]
    lea rsi, [r9 + hand.cards]
    mov rcx, NUM_CARDS
    repe cmpsb
    jb .greater
    ja .less

    ret
    .greater:
    inc rax
    ret
    .less:
    dec rax
    ret

    %else

    xor rax, rax
    xor rcx, rcx

    .run_loop:
        mov dl, byte [rdi + hand.runs + rcx]
        cmp dl, byte [rsi + hand.runs + rcx]
        ja .greater
        jb .less

        inc rcx
        cmp rcx, NUM_CARDS
        jb .run_loop

    xor rcx, rcx
    .card_loop:

        mov dl, byte [rdi + hand.cards + rcx]
        cmp dl, byte [rsi + hand.cards + rcx]
        ja .greater
        jb .less

        inc rcx
        cmp rcx, NUM_CARDS
        jb .card_loop
    ret

    .greater:
    inc rax
    ret
    .less:
    dec rax
    ret

    %endif


exit:
    mov rax, 60
    syscall
    trap

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
    trap

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


global _start
_start:

    ; size of line buffer
    mov rbx, 24
    ; line buffer
    sub rsp, rbx
    mov r12, rsp

    ; total number of hands
    xor r13, r13

    .loop_input:
        mov rdi, r12
        mov rsi, rbx
        call read_line

        test rax, rax
        jz .done_reading_input

        mov rdi, r12
        mov rsi, rax
        call hand_parse

        test rax, rax
        jz .error

        mov rdi, rax
        %if PROBLEM = 1
        call hand_fill_runs1
        %elif PROBLEM = 2
        call hand_fill_runs2
        %else
        %error "unknown PROBLEM"
        %endif

        inc r13
        jmp .loop_input

    .done_reading_input:

    lea rdi, [rel hands]
    mov rsi, r13
    mov rdx, hand_size
    lea rcx, [rel hand_compare]
    call insertion_sort

    ; aggregate in rbx
    xor rbx, rbx

    ; counter in rcx
    xor rcx, rcx

    lea rdi, [rel hands]
    .aggregate_loop:
        cmp rcx, r13
        jae .print_results

        inc rcx

        mov rax, rcx
        mov rdx, qword [rdi + hand.bids]
        mul rdx

        add rbx, rax

        add rdi, hand_size
        jmp .aggregate_loop

    .print_results:

    mov rdi, rbx
    mov rsi, r12
    mov rdx, 24
    call utoa

    mov rdi, r12
    mov rsi, rax
    call print

    mov rdi, r12
    mov byte [rdi], 0x0a
    mov rsi, 1
    call print

    xor rdi, rdi
    call exit

    .error:
    mov rdi, 1
    call exit
