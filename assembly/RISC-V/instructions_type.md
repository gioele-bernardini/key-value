### Istruzioni di tipo R
```
funct7  rs2  rs1  funct3  rd  opcode
7 bit   5 bit 5 bit 3 bit 5 bit 7 bit
```
- **funct7**: Campo funzione a 7 bit per ulteriori dettagli sull'operazione.
- **rs2**: Registro sorgente 2 (5 bit).
- **rs1**: Registro sorgente 1 (5 bit).
- **funct3**: Campo funzione a 3 bit per specificare l'operazione.
- **rd**: Registro di destinazione (5 bit).
- **opcode**: Codice operazione a 7 bit per identificare la categoria principale dell'istruzione.

### Esempio di istruzione di tipo R
```
add x3, x1, x2
```
```
funct7   rs2      rs1      funct3  rd       opcode
0110011  00010   00001   000      00011   0110011
```

### Istruzioni di tipo I
```
imm[11:0]  rs1  funct3  rd  opcode
12 bit     5 bit 3 bit  5 bit 7 bit
```
- **imm[11:0]**: Immediato a 12 bit.
- **rs1**: Registro sorgente 1 (5 bit).
- **funct3**: Campo funzione a 3 bit per specificare l'operazione.
- **rd**: Registro di destinazione (5 bit).
- **opcode**: Codice operazione a 7 bit per identificare la categoria principale dell'istruzione.

### Esempio di istruzione di tipo I
```
addi x5, x6, 10
```
```
imm[11:0]   rs1    funct3    rd     opcode
000000001010  00110  000      00101  0010011
```

### Istruzioni di tipo S
```
imm[11:5]  rs2  rs1  funct3  imm[4:0]  opcode
7 bit      5 bit 5 bit 3 bit 5 bit     7 bit
```
- **imm[11:5]**: Parte alta dell'immediato (7 bit).
- **rs2**: Registro sorgente 2 (5 bit).
- **rs1**: Registro sorgente 1 (5 bit).
- **funct3**: Campo funzione a 3 bit per specificare l'operazione.
- **imm[4:0]**: Parte bassa dell'immediato (5 bit).
- **opcode**: Codice operazione a 7 bit per identificare la categoria principale dell'istruzione.

### Esempio di istruzione di tipo S
```
sw x5, 8(x6)
```
```
imm[11:5]   rs2    rs1    funct3    imm[4:0]    opcode
0000000     00101  00110  010       01000       0100011
```

