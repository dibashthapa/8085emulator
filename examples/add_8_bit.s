;(2501 H) = 99H 
;(2502 H) = 39H 
;Result (2503 H) = 99H + 39H = D2H  
;Since,
;   1 0 0 1 1 0 0 1 (99H) 
; + 0 0 1 1 1 0 0 1 (39H) 
;   1 1 0 1 0 0 1 0 (D2H)
LXI H, 2501H  ; "Get address of first number in H-L pair. Now H-L points to 2501H"  
MOV A, M      ; "Get first operand in accumulator"  
INX H         ; "Increment content of H-L pair. Now, H-L points 2502H"  
ADD M         ; "Add first and second operand"  
INX H         ; "H-L points 4002H"  
MOV M, A      ; "Store result at 2503H"  
HLT           ; "Stop"  