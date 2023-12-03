;Find the square of 07 (decimal) using lookup table techniques.
;
;The number 07 D is in the memory.
;
;The result is to be stored in the memory location 2501H.
;
;The table for square is stored from 2600 to 2609 H.
LDA 2500H   ; "Get data in accumulator"  
MOV L, A    ; "Get data in register L"  
MVI H, 26H  ; "Get 26 in register H"  
MOV A, M    ; "Square of data in accumulator"  
STA 2501 H  ; "Store Square in 2501 H"  
HLT ; "Stop"  