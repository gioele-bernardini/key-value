section .bss
  sum resb 1

section .data
  msg db 'The sum is : '
  len equ $ - msg
  newline db 10

section .text
global _start

_start:
  mov eax, 4
  mov ebx, 2

  add eax, ebx
  add eax, '0'
  mov [sum], eax

  ; Printing the message
  mov edx, len
  mov ecx, msg
  mov ebx, 1
  mov eax, 4
  int 0x80

  ; Printing the sum
  mov edx, 1
  mov ecx, sum
  mov ebx, 1
  mov eax, 4
  int 0x80

  ; Printing the new-line character
  mov eax, 4
  mov ebx, 1
  mov ecx, newline
  mov edx, 1
  int 0x80

  mov ebx, 0
  mov eax, 1
  int 0x80
