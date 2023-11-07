# sagakar-task-7
## Reference sheet
### Instruction set
|Type|Name|Description                                              |
|----|----|---------------------------------------------------------|
| A  |AND |Bitwise AND Arg2 and Arg3 and save in Arg1               |
| N  |NOT |Bitwise NOT Arg2 and save in Arg1                        |
| A  |OR  |Bitwise OR Arg2 and Arg3 and save in Arg1                |
| A  |XOR |Bitwise XOR Arg2 and Arg3 and save in Arg1               |
| A  |SHR |Shift Arg2 right by Arg3 positions and save in Arg1      |
| A  |SHL |Shift Arg2 left by Arg3 positions and save in Arg1       |
| A  |ADD |Add Arg2 and Arg3 and save in Arg1                       |
| A  |SUB |Subtract Arg3 from Arg2 and save in Arg1                 |
| M  |LOAD|Set Arg1 to the value at address Arg2 offset by Arg3     |
| M  |SET |Set the value at address Arg2 to Arg1 offset by Arg3     |
| I  |LDIM|Set IM to the immediate Arg1                             |
| A  |JEQ |Jump to the address in Arg1 if Arg2 = Arg3               |
| A  |JLT |Jump to the address in Arg1 if Arg2 < Arg3               |
| A  |JGT |Jump to the address in Arg1 if Arg2 > Arg3               |
| J  |JAR |Jump to the address in Arg1, store current PC value in RA|
| N  |MOV |Copy the contents of Arg1 to Arg2                        |

### Instruction types
|Type  |Opcode         |Arg1               |Arg2             |Arg3              |Total size|
|------|---------------|-------------------|-----------------|------------------|----------|
|A-type|opcode (4 bits)|register (3 bits)  |register (3 bits)|register (3 bits) |13 bits   |
|I-type|opcode (4 bits)|immediate (12 bits)|                 |                  |16 bits   |
|J-type|opcode (4 bits)|register (3 bits)  |                 |                  |7 bits    |
|M-type|opcode (4 bits)|register (3 bits)  |register (3 bits)|immediate (6 bits)|16 bits   |
|N-type|opcode (4 bits)|register (3 bits)  |register (3 bits)|                  |10 bits   |

### Registers
|Name|Description                                                               |
|----|--------------------------------------------------------------------------|
|R0  |General purpose                                                           |
|R1  |General purpose                                                           |
|R2  |General purpose                                                           |
|R3  |General purpose                                                           |
|R4  |General purpose                                                           |
|RA  |JAR return address                                                        |
|IM  |12-bit immediates loaded through LDIM                                     |
|IO  |Input/output. <br /> Input: reads a utf-8 character from stdin with every read iterating one character <br /> Output: any data saved here is printed as a utf-8 character to stdout|