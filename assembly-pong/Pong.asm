STACK SEGMENT PARA STACK
  ; Define Byte (DB), 64 time duplicate (' ')
  DB 64 DUP (' ')
STACK ENDS

DATA SEGMENT PARA 'DATA'

DATA ENDS

CODE SEGMENT PARA 'CODE'

  MAIN PROC FAR
  
    MOV AH, 00h ; Set the configuration to video mode
    MOV AL, 13h ; Choose the video mode
    INT 10h ; Execute the configuration

    MOV AH, 0Bh ; Set the configuration
    MOV BH, 00h ; To the background color
    MOV BL, 00h ; Choose black as background color
    INT 10h ; Execute the configuration

    MOV AH, 0Ch ; Set the configuration to writing a pixel
    MOV AL, 0Fh ; Choose white as color
    MOV BH, 00h ; Set the page number
    MOV CX, 0Ah ; Set the column (X position)
    MOV DX, 0Ah ; Set the line (Y position)
    INT 10h

    RET
  MAIN ENDP

CODE ENDS
END
