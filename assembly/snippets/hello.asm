section .data
  hello_msg db 'Hello, World!', 10, 0

section .text
  global _start

_start:
  mov rax, 1
  mov rdi, 1
  mov rsi, hello_msg
  mov rdx, 14
  syscall

  mov rax, 60
  xor rdi, rdi
  syscall