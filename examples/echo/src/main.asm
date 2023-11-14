.title example program

.include std.inc
.include tty.inc

stack = 10000
ustack = 16000

t_exit = 1 ; my custom trap id: exit user process
t_max = 1 ; max syscall

.asect
.= 1000
    mov #stack,sp
    mov #input, @#v_tti  ; install keyboard ISR
    bis #100, @#kbs     ; enable keyboard interrupts

    mov #on_cpu_err, @#v_cpu
    mov #on_ill, @#v_ill
    mov #on_trap, @#v_trap

    bis #0144000, @#ps ; user mode + register set 1
    mov #ustack,sp

    call main
    halt
    halt

main:
    mov #intro, r0
    call print

    mov @#ps, r0
    call print_oct

1$:
    mov #prompt, r0
    call print

    call read_line
    cmp #005161, (r0) ; q + \r
    beq 2$

    ; echo
    call print

    br 1$
2$:
    trap #t_exit
    nop
    nop

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

on_trap:
    mov (sp), r0
    add #-2, r0
    movb (r0), r0
    cmp r0, #t_max
    bgt s_bad
    asl r0
    call @syscalls(r0)
    rti
on_cpu_err:
    mov #m_cpu_err, r0
    call print
    mov @#ERR, r0
    call print_oct
    br err_at
on_ill:
    mov #m_ill, r0
    call print
    br err_at
err_at:
    mov #m_at, r0
    call print
    mov (sp), r0
    add #-2, r0
    call print_oct
    br fatal
fatal:
    halt
    nop

s_exit:
    mov #bye, r0
    call print
    halt
    nop

s_bad:
    mov #m_bad_s, r0
    call print
    halt
    nop

syscalls:
    .word 0
    .word s_exit

intro:   .asciz  /This is a dummy unix shell:/<NL>
prompt:  .asciz  /$ /
bye:     .asciz  /bye/<NL>
tick:    .asciz  /./

m_cpu_err: .asciz /E: CPU ERR: /
m_ill:   .asciz /E: Illegal instruction/
m_at:    .asciz / at /
m_bad_s: .asciz /E: bad sycall/

.even
indone: .word 0 ; non-zero when a line is ready 
inpos:  .word 0 ; pointer in inbuf where to write next character
inbuf:  .BLKB 200

    .end
