;2500 H = 4H
;2501 H = 20H
;2502 H = 15H
;2503 H = 13H
;2504 H = 22H
Result = 2505 H = 20+22= 42H
;The numbers are placed in the memory locations 2501 to 2504H.
;
;The sum is to be stored in the memory location 2450H.
;
;As there are 4 numbers in the series, count = 04
;
;The initial value of the sum is made 00. The even number of the series are taken one by one and added to the sum
LDA 2500H  
MOV C, A            ; "Initialize counter"  
MVI B, 00H          ; "sum = 0"  
LXI H, 2501H        ; "Initialize pointer"  
BACK: MOV A, M      ; "Get the number"  
ANI 01H             ; "Mask Bit l to Bit7"  
JNZ SKIP            ; "Don't add if number is ODD"  
MOV A, B            ; "Get the sum"  
ADD M               ; "SUM = SUM + data"  
MOV B, A            ; "Store result in B register"  
SKIP: INX H         ; "increment pointer"  
DCR C               ; "Decrement counter"  
JNZ BACK            ; "if counter 0 repeat"  
STA 2505H           ; "store sum"  
HLT                 ; "Stop"  