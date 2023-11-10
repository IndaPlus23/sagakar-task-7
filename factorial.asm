$ascii_offset '0'
$null_byte 0

// Convert from ascii input to integer
(READ_LOOP)
    MOV R0 IO
    LDIM 0
    MOV R1 IM
    LDIM READ_END
    JEQ IM R0 R1
    MOV R3 R2
    LDIM 3
    SHL R2 R2 IM
    LDIM 1
    SHL R3 R3 IM
    ADD R2 R2 R3
    LDIM $ascii_offset
    SUB R0 R0 IM
    ADD R2 R2 R0
    LDIM READ_LOOP
    JAR IM
(READ_END)
MOV R0 R2 // Move result to R0
LDIM $ascii_offset
MOV R4 R0
ADD R4 R4 IM
MOV IO R4 
// 12! is the largest factorial that fits in an unsigned 32 bit integer
// Don't attempt to calculate if n > 12
LDIM 12
MOV R1 IM
LDIM N_TOO_LARGE
JGT IM R0 R1

//LDIM MULTIPLY
//JAR IM
//MOV IO R2
//LDIM END
//JAR IM

LDIM END
JAR IM

// int a -> R0, int b -> R1
(MULTIPLY)
    LDIM 0
    MOV R2 IM // int sum = 0
    MOV R3 IM // int i = 0
    LDIM SKIP_MULT_LOOP
    JLT IM R0 R3
    (MULT_LOOP)
        ADD R2 R2 R1
        LDIM 1
        ADD R3 R3 IM
        LDIM MULT_LOOP
        JLT IM R3 R0
    (SKIP_MULT_LOOP)
    JEQ RA IM IM


// Bit shift divide by 10 algorithm from Hacker's Delight
// https://web.archive.org/web/20180517023231/http://www.hackersdelight.org/divcMore.pdf
(DIVIDE_BY_TEN)
    // Assume R0 = n
    // q = n >> 1 + n >> 2
    MOV R1 R0 // Retain original value of n
    MOV R2 R0
    MOV R3 R0 // retain n for later use
    LDIM 0
    MOV R0 IM // Clear n from R0 to use for q
    LDIM 1
    SHR R1 R1 IM
    LDIM 2
    SHR R2 R2 IM
    ADD R0 R1 R2
    // q = q + (q >> 4)
    MOV R1 R0
    LDIM 4
    SHR R1 R1 IM
    ADD R0 R0 R1
    // q = q + (q >> 8)
    MOV R1 R0
    LDIM 8
    SHR R1 R1 IM
    ADD R0 R0 R1
    // q = q + (q >> 16)
    MOV R1 R0
    LDIM 16
    SHR R1 R1 IM
    ADD R0 R0 R1
    // q = q >> 3
    LDIM 3
    SHR R0 R0 IM
    // r = n - (((q << 2) + q) << 1)
    MOV R1 R0
    MOV R2 R0
    LDIM 2
    SHL R1 R1 IM
    ADD R1 R1 R2
    LDIM 1
    SHL R1 R1 IM
    SUB R1 R3 R1
    // return q + (r > 9) <==> return q + !(r < 10)
    LDIM 10
    MOV R2 IM
    LDIM DIVIDE_BY_TEN_END
    JLT IM R1 R2 // q = q if !(r < 10) = false
    ADD R0 R0 R1 // q = q + r if !(r < 10) = true
(DIVIDE_BY_TEN_END)
JEQ RA IM IM

(N_TOO_LARGE)
    LDIM END
    JAR IM

(END)