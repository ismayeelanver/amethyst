section .data
    msg db 'Hello, world!', 0xa, 0x0  ; Message to print with a newline

section .text
    global _start

_start:
    ; Write the message to stdout
    mov rax, 1            ; syscall: sys_write
    mov rdi, 1            ; file descriptor: stdout
    mov rsi, msg          ; pointer to the message
    mov rdx, 14           ; length of the message
    syscall

    ; Exit the program
    mov rax, 60           ; syscall: sys_exit
    xor rdi, rdi          ; exit code: 0
    syscall
