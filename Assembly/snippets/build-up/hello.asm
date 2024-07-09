section .data
  hello db 'Hello, World!',10
  hello_len equ $-hello

section .text
  global _start

_start:
  ; STDOUT syscall
  mov eax, 1
  mov edi, 1
  mov rsi, hello
  mov edx, hello_len
  syscall

  mov eax, 60
  xor edi, edi
  syscall
