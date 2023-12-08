[bits 64]

%include "./lib.asm"

%define trap db 0xcc

struc hashtable_entry
    .key: resq 1
    .value: resq 1
endstruc

struc hashtable
    .capacity: resq 1
    .num_entries: resq 1
    .hash_function: resq 1
    .key_compare: resq 1
    .entries:
endstruc

%define SIZEOF_HASHTABLE(capacity) (hashtable_size + (capacity) * hashtable_entry_size)

section .bss
    nodes: resb SIZEOF_HASHTABLE(2048)

    directions: resb 512
    num_directions: resq 1

section .text

; int compare_u32(u32 a, u32 b)
; edi - a
; esi - b
; -> return -1, 0, 1 if (a < b), (a == b), (a > b) respectively
compare_u32:
    xor rax, rax
    cmp edi, esi
    ja .greater
    jb .lesser
    ret

    .greater:
    inc rax
    ret

    .lesser:
    dec rax
    ret

; u64 hash_u32(u32 value)
; simple fibonacci hash function
; rdi - value
; -> return hash
hash_u32:
    mov rax, 11400714819323198485
    mul rdi
    ret


; struct hashtable* hashtable_init(struct hashtable* ht,
;                                  u64 capacity,
;                                  u64 (*hash_function)(const void*),
;                                  int (*key_compare)(const void*, const void*))
; initialize a hashtable with the given capacity and hash function
; rdi - hashtable to initialize
; rsi - capacity of the hashtable
; rdx - hash function to use for the hashtable
; rcx - key compare function to use
; -> return the hashtable pointer
hashtable_init:
    mov r8, rdi
    mov r9, rcx

    %if (hashtable_size % 8) != 0 || (hashtable_entry_size % 8) != 0
    %error "hashtable size is not 8 byte aligned"
    %endif

    lea rcx, [rsi * (hashtable_entry_size / 8) + (hashtable_size / 8)]
    xor rax, rax
    rep stosq

    mov qword [r8 + hashtable.capacity], rsi
    mov qword [r8 + hashtable.hash_function], rdx
    mov qword [r8 + hashtable.key_compare], r9

    mov rax, r8
    ret

; const struct hashtable_entry* hashtable_search(struct hashtable* ht, void* key)
; check whether the given entry is present in the hashtable.
; rdi - hashtable ptr
; rsi - key to search
; -> return the entry found or NULL
hashtable_search:
    push r15
    push r14
    push r13
    push r12

    mov r15, rdi
    mov r14, rsi

    ; calculate the index according to hash function
    mov rdi, r14
    call qword [r15 + hashtable.hash_function]
    mov r12, rax

    .probe:
    ; hash in r12, modulo in rdx:
    mov rax, r12
    xor rdx, rdx
    div qword [r15 + hashtable.capacity]

    ; calculate offset
    mov rax, hashtable_entry_size
    mul rdx

    ; hashtable_entry* in r13
    lea r13, [r15 + rax + hashtable.entries]

    ; get the key
    mov rdi, [r13 + hashtable_entry.key]
    cmp rdi, 0
    jz .not_found

    ; check if this is the right entry
    mov rsi, r14
    call qword [r15 + hashtable.key_compare]

    test rax, rax
    jz .found

    inc r12
    jmp .probe

    .not_found:
    xor rax, rax
    jmp .return

    .found:
    mov rax, r13

    .return:
    pop r12
    pop r13
    pop r14
    pop r15
    ret

; bool hashtable_upsert(struct hashtable* ht, void* key, void* value)
; insert the given entry into the hashtable. Update the value if the key exists
; rdi - hashtable ptr
; rsi - key to insert
; rdx - associated value to insert
; -> return whether the entry was inserted
hashtable_upsert:
    push r15    ; hashtable
    push r14    ; key
    push r13    ; currently probed slot
    push r12    ; current index
    push rbx    ; value

    mov r15, rdi

    mov rdi, qword [r15 + hashtable.num_entries]
    cmp rdi, qword [r15 + hashtable.capacity]
    jae .full

    ; save off entry to insert
    mov r14, rsi
    mov rbx, rdx

    ; calculate the index according to hash function
    mov rdi, r14
    call qword [r15 + hashtable.hash_function]
    mov r12, rax

    .probe:
    ; hash in r12, modulo in rdx:
    mov rax, r12
    xor rdx, rdx
    div qword [r15 + hashtable.capacity]

    ; calculate offset
    mov rax, hashtable_entry_size
    mul rdx

    ; hashtable_entry* in r13
    lea r13, [r15 + rax + hashtable.entries]

    ; get the key
    mov rdi, [r13 + hashtable_entry.key]
    cmp rdi, 0
    jz .insert

    ; check if the entry already exists
    mov rsi, r14
    call qword [r15 + hashtable.key_compare]

    test rax, rax
    jz .upsert

    inc r12
    jmp .probe

    .insert:
    inc qword [r15 + hashtable.num_entries]
    mov qword [r13 + hashtable_entry.key], r14
    .upsert:
    mov qword [r13 + hashtable_entry.value], rbx

    mov rax, 1
    jmp .return

    .full:
    xor rax, rax

    .return:
    pop rbx
    pop r12
    pop r13
    pop r14
    pop r15
    ret


; void parse_direction(char* str, u64 len)
; process the direction instruction in place, replace them with internal values
; L => 0
; R => 0xFF
; rdi - string to parse / replace
; rsi - length of string
parse_direction:
    xor rcx, rcx

    .next:
        cmp byte [rdi + rcx], 'L'
        sete byte [rdi + rcx]
        dec byte [rdi + rcx]

        inc rcx
        cmp rcx, rsi
        jb .next

    ret


; key:value parse_node(const char* str, u64 len)
; parse key value pair of the input.
; rdi - string to parse
; rsi - string length
; output structure will be rax = nodeid, (rdx[0:32], rdx[32:64]) = (left, right)
parse_node:
    cmp rsi, 16
    jne .error

    %macro read_node_id 2
    movzx r8d, byte [rdi + %1 + 0]
    or %2, r8d

    movzx r8d, byte [rdi + %1 + 1]
    shl r8d, 8
    or %2, r8d

    movzx r8d, byte [rdi + %1 + 2]
    shl r8d, 16
    or %2, r8d
    %endmacro

    xor rax, rax
    read_node_id 0, eax

    xor rsi, rsi
    read_node_id 7, esi

    xor rdx, rdx
    read_node_id 12, edx

    shl rdx, 32
    or rdx, rsi
    ret

    .error:
    xor rax, rax
    ret


global _start
_start:
    ; first initialize the nodes hash table.
    ; we inline the keys and the values as they fit into 8 bytes
    lea rdi, [rel nodes]
    mov rsi, 2048
    lea rdx, [rel hash_u32]
    lea rcx, [rel compare_u32]
    call hashtable_init

    ; load the direction instructions
    lea rdi, [rel directions]
    mov rsi, 512
    call read_line
    mov qword [rel num_directions], rax

    lea rdi, [rel directions]
    mov rsi, rax
    call parse_direction

    ; line buffer
    sub rsp, 24

    ; skip the empty line
    mov rdi, rsp
    mov rsi, 24
    call read_line

    .next_line:
        mov rdi, rsp
        mov rsi, 24
        call read_line

        test rax, rax
        jz .done_reading_input

        mov rdi, rsp
        mov rsi, rax
        call parse_node

        ; key in rax
        ; value in rdx
        test rax, rax
        jz .error

        lea rdi, [rel nodes]
        mov rsi, rax
        call hashtable_upsert

        test rax, rax
        jz .error

        jmp .next_line

    .done_reading_input:

    ; current node id
    mov r15, 'AAA'
    ; index in directions / step count
    xor r14, r14
    lea r13, [rel directions]

    .next_step:
        cmp r15, 'ZZZ'
        je .done

        lea rdi, [rel nodes]
        mov rsi, r15
        call hashtable_search

        test rax, rax
        jz .error

        mov r8, qword [rax + hashtable_entry.value]

        mov rax, r14
        xor rdx, rdx
        div qword [rel num_directions]
        ; modulo index in rdx

        ; direction mask in al
        mov al, byte [r13 + rdx]
        ; select either right or left based on direction mask
        ; mask is 0xFF for right, 0 for left
        mov cl, 32
        and cl, al
        shr r8, cl

        ; select next node
        mov r15d, r8d

        inc r14
        jmp .next_step

    .done:

    mov rdi, r14
    mov rsi, rsp
    mov rdx, 24
    call utoa

    mov rdi, rsp
    mov rsi, rax
    call print

    mov rdi, rsp
    mov byte [rdi], 0x0a
    mov rsi, 1
    call print

    xor rdi, rdi
    call exit

    .error:
    mov rdi, 1
    call exit
    trap
