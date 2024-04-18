section .data
  num1 dd 20
  num2 dd 10
  result dd 0

section .text
  global _start

_start:
  mov eax, [num1]  ; Load value of num1 into EAX
  sub eax, [num2]  ; Add value of num2 to EAX (sum)
  mov [result], eax ; Store the sum in result

  mov rax, 1
  mov rdi, 1
  mov rsi, result
  mov rdx, 2
  syscall

  xor eax, eax
  xor edi, edi

  mov eax, 60
  xor edi, edi
  syscall
