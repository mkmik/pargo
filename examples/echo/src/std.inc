CR = 15
LF = 12
NL = 12

PS = 777776 ; process status register
ERR = 777766 ; error register


; trap vectors
V_CPU = 004 ; CPU error
V_ILL = 010 ; Illegal & reserved instructions
V_BPT = 014 ; breakpoint trap
V_IOT = 020 ; input output trap
V_PWR = 024 ; power fail
V_EMT = 030 ; emulator trap
V_TRAP = 034 ; trap instructions

V_SYS1 = 040 ; system software
V_SYS2 = 044 ; system software
V_SYS3 = 050 ; system software
V_SYS4 = 054 ; system software

V_TTI = 060 ; Console Terminal, keyboard/reader
V_TTO = 064 ; Console Terminal, printer/punch

V_PTR = 070 ; PC11, paper tape reader
V_PTP = 074 ; PCl1, paper tape punch
V_LCLK = 100 ; KW11-L, line clock
V_PCLK = 104 ; KW11-P, programmable clock 

V_LPR = 200 ; LPll/LSl1, line printer
