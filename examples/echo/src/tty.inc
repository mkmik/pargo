kbs = 0177560            ; keyboard status register
kbd = 0177562            ; keyboard data buffer

tts = 0177564            ; terminal status register
ttd = 0177566            ; terminal data buffer

.macro putch, r
    putch_nw r  
    flushout
.endm

.macro putch_nw, r
    movb r, @#ttd   
.endm

.macro flushout
    tstb @#tts
    bpl .-4
.endm
