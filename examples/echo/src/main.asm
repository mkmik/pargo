.title example program

.include std.inc
.include tty.inc

stack = 10000
ustack = 16000

.asect
.= 1000
    mov #stack,sp
    bis #0140000, @#ps
    mov #ustack,sp

    mov #input, @#kbv   ; install keyboard ISR
    bis #100, @#kbs     ; enable keyboard interrupts

    call main
    halt
    halt

main:
    mov #intro, r0
    call print

1$:
    mov @#ps, r0
    call print_oct

    mov #123, r0
    call print_oct


    mov #prompt, r0
    call print

    call read_line
    cmp #005161, (r0) ; q + \r
    beq 2$

    ; echo
    call print

    br 1$
2$:
    mov #bye, r0
    call print

    rts pc

; prints a null terminated string. Converts \n into \r\n
print:
    movb (r0)+, r1
    cmp r1, #NL
    bne 1$
    putch #CR
1$:
    putch r1
    tst r1
    bne print
    rts pc

print_oct:
    mov #-017, r1
1$:
    mov r0, r3
    clr r2
    ashc r1,r2
    bic #177770, r3
    add #'0, r3
    putch r3
    add #3, r1
    ble 1$
    putch #CR
    putch #LF
    rts pc

input:
    ;mov @#ps, r0
    ;call print_oct

    movb @#kbd, r1
    putch r1
    cmp r1, #CR
    bne 1$
    putch #LF
    movb #LF, r1
    inc @#indone
1$:
    mov @#inpos, r2
    movb r1, (r2)+
    movb 0, (r2)
    mov r2, @#inpos
    rti

read_line:
    clr @#indone
    mov #inbuf, @#inpos
1$:
    tst @#indone
    bne 2$
    wait
    br 1$
2$:
    mov #inbuf, r0
    rts pc

intro:   .asciz  /This is a dummy unix shell:/<NL>
prompt:  .asciz  /$ /
bye:     .asciz  /bye/<NL>
tick:    .asciz  /./

.even
indone: .word 0 ; non-zero when a line is ready 
inpos:  .word 0 ; pointer in inbuf where to write next character
inbuf:  .BLKB 200

    .end
