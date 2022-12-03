.align 8
.global read_input
read_input:
    stp x29, x30, [sp, #-64]!
    str x19, [sp, #16]
    str x20, [sp, #24]
    str x21, [sp, #32]

    // open the "input.txt" as "rb"
    adr x0, input_path
    adr x1, open_param
    bl fopen
    mov x19, x0

    // go to the end of file
    mov x1, #0
    mov x2, #2
    bl fseek

    // get the size of the file by determining
    // our position within it
    mov x0, x19
    bl ftell
    mov x20, x0

    // go back to the start of the file
    mov x0, x19
    mov x1, #0
    mov x2, #0
    bl fseek

    // allocate memory to store the file
    mov x0, x20
    add x0, x0, #1
    mov x1, #1
    bl calloc
    mov x21, x0

    // read the contents into memory
    mov x1, #1
    mov x2, x20
    mov x3, x19
    bl fread

    // close the file
    mov x0, x19
    bl fclose

    mov x0, x21
    ldr x19, [sp, #16]
    ldr x20, [sp, #24]
    ldr x21, [sp, #32]
    ldp x29, x30, [sp], #64
    ret

input_path:
.string "input.txt"

open_param:
.string "rb"

.align 8
.global alpha_to_index
alpha_to_index:
    stp x29, x30, [sp, #-32]!
    str x19, [sp, #16]
    str x20, [sp, #24]
    mov w19, w0

    mov w0, w19
    bl isupper
    mov w20, w0

    mov w0, w19
    bl tolower

    add w0, w0, #-'a'

    cbz w20, ati_skip_uppercase_amendment
    add w0, w0, #26

ati_skip_uppercase_amendment:
    ldr x19, [sp, #16]
    ldr x20, [sp, #24]
    ldp x29, x30, [sp], #32
    ret

.align 8
.global populate_alpha_bit
populate_alpha_bit:
    stp x29, x30, [sp, #-64]!
    str x19, [sp, #16]
    str x20, [sp, #24]
    str x21, [sp, #32]
    str x22, [sp, #40]

    mov x19, x0 // alphabuffer
    mov x20, x1 // input string
    mov x21, x2 // input length
    mov x22, x3 // bit to set

    // set x22 to the bit value
    mov x0, #1
    lsl x22, x0, x22

pab_loop:
    // if there is no more input then exit
    cbz x21, pab_exit

    # read the next letter in the input
    # and get the alphabuffer index
    ldrb w0, [x20]
    bl alpha_to_index

    # set the corresponding bit in the buffer
    ldrb w1, [x19, x0]
    orr w1, w1, w22
    strb w1, [x19, x0]

    # increment and decrement the pointer and size
    # respectfully
    add x20, x20, #1
    add x21, x21, #-1

    # repeat
    b pab_loop

pab_exit:
    ldr x19, [sp, #16]
    ldr x20, [sp, #24]
    ldr x21, [sp, #32]
    ldr x22, [sp, #40]
    ldp x29, x30, [sp], #64
    ret

.align 8
.global find_bit_count
find_bit_count:
    stp x29, x30, [sp, #-64]!
    str x19, [sp, #16]
    str x20, [sp, #24]
    str x21, [sp, #32]
    str x22, [sp, #40]

    // x0 - alphabet buffer
    // x1 - num bits to match
    eor x2, x2, x2 // index
    mov x3, #52 // max size
    mov x19, x0
    mov x20, x1
    mov x21, x2
    mov x22, x3

fbc_loop:
    ldrb w0, [x19, x21]
    rbit w0, w0
    mvn w0, w0
    clz w0, w0
    sub w0, w0, w20
    cbz w0, fbc_found
    add x21, x21, #1
    sub x4, x22, x21
    cbnz x4, fbc_loop
    b fbc_error

fbc_found:
    mov x0, x21
    ldr x19, [sp, #16]
    ldr x20, [sp, #24]
    ldr x21, [sp, #32]
    ldr x22, [sp, #40]
    ldp x29, x30, [sp], #64
    ret

fbc_error:
    adr x0, fbc_error_msg
    bl puts
    mov x0, 1
    bl exit

check_loop:
.string "%x %x %x\n"
fbc_error_msg:
.string "Error, no bit count found\n"

.align 8
.global part1
part1:
    stp x29, x30, [sp, #-128]!
    str x19, [sp, #16]
    str x20, [sp, #24]
    str x21, [sp, #32]
    str x22, [sp, #40]
    str x23, [sp, #48]

    // keep the input stored in x19
    mov x19, x0 // input
    mov x20, #0 // total
    add x21, sp, #64 // alphabet buffer

p1_loop_through_lines:
    // loop through each line of the input
    adr x1, newline_token
    bl strtok

    // check if we're at the end
    cbz x0, p1_exit
    mov x22, x0

    // clear our alphabet buffer
    mov x0, x21
    eor x1, x1, x1
    mov x2, #52
    bl memset

    // get the length of the combined compartments
    mov x0, x22
    bl strlen

    // each compartment is half the size
    lsr x23, x0, #1

    // populate the first input compartment with bit 0
    mov x0, x21 // alphabet buffer
    mov x1, x22 // string
    mov x2, x23 // string length
    mov x3, #0 // bit number
    bl populate_alpha_bit

    // populate the second input compartment with bit 1
    mov x0, x21 // alphabet buffer
    add x1, x22, x23 // string
    mov x2, x23 // string length
    mov x3, #1 // bit number
    bl populate_alpha_bit

    // find any entry with 2 bits set
    mov x0, x21 // alphabet buffer
    mov x1, #2
    bl find_bit_count

    // add the index + 1 to the total
    add x20, x20, x0
    add x20, x20, #1

    // continue to the next line
    mov x0, #0
    b p1_loop_through_lines
    
p1_exit:
    mov x1, x20
    adr x0, format_string_int
    bl printf

    ldr x19, [sp, #16]
    ldr x20, [sp, #24]
    ldr x21, [sp, #32]
    ldr x22, [sp, #40]
    ldr x23, [sp, #48]
    ldp x29, x30, [sp], #128
    ret

.align 8
.global part2
part2:
    stp x29, x30, [sp, #-128]!
    str x19, [sp, #16]
    str x20, [sp, #24]
    str x21, [sp, #32]
    str x22, [sp, #40]

    // keep the input stored in x19
    mov x19, x0 // input file
    mov x20, #0 // total count
    add x21, sp, #64 // alphabet buffer

p2_loop_through_lines:
    // loop through each line of the input
    adr x1, newline_token
    bl strtok

    // check if we're at the end
    cbz x0, p2_exit
    mov x22, x0

    // clear our alphabet buffer
    mov x0, x21
    eor x1, x1, x1
    mov x2, #52
    bl memset

    // get the string length
    mov x0, x22
    bl strlen

    // populate the first input alphabet with bit 0
    mov x2, x0 // string length
    mov x0, x21 // alphabet buffer
    mov x1, x22 // string
    mov x3, #0 // bit number
    bl populate_alpha_bit

    // get the next input
    mov x0, #0
    adr x1, newline_token
    bl strtok
    mov x22, x0

    // get the string length
    mov x0, x22
    bl strlen

    // populate the first input alphabet with bit 1
    mov x2, x0 // string length
    mov x0, x21 // alphabet buffer
    mov x1, x22 // string
    mov x3, #1 // bit number
    bl populate_alpha_bit

    // get the next input
    mov x0, #0
    adr x1, newline_token
    bl strtok
    mov x22, x0

    // get the string length
    mov x0, x22
    bl strlen

    // populate the first input alphabet with bit 2
    mov x2, x0 // string length
    mov x0, x21 // alphabet buffer
    mov x1, x22 // string
    mov x3, #2 // bit number
    bl populate_alpha_bit

    // find any entry with 3 bits set
    mov x0, x21 // alphabet buffer
    mov x1, #3
    bl find_bit_count

    // add the index + 1 to the total
    add x20, x20, x0
    add x20, x20, #1

    // continue to the next line
    mov x0, #0
    b p2_loop_through_lines
    
p2_exit:
    mov x1, x20
    adr x0, format_string_int
    bl printf

    ldr x19, [sp, #16]
    ldr x20, [sp, #24]
    ldr x21, [sp, #32]
    ldr x22, [sp, #40]
    ldp x29, x30, [sp], #128
    ret

format_string_int:
.string "%i\n"

newline_token:
.string "\n"

.align 8
.global main
main:
    stp x29, x30, [sp, #-32]!
    str x19, [sp, #16]

    // read the input.txt to a null terminated string
    bl read_input
    mov x19, x0
    bl strdup
    bl part1
    mov x0, x19
    bl part2
    ldr x19, [sp, #16]
    ldp x29, x30, [sp], #32
    ret
