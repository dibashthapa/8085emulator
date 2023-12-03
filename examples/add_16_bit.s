;(2501H) = 15H
;(2502H) = 1CH
;(2503H) = B7H
;(2504H) = 5AH
;
;Result = 1C15 + 5AB7H = 76CCH
;
;(2505H) = CCH
;(2506H) = 76H
LHLD 2501H   ; "Get 1st 16-bit number in H-L pair"  
XCHG         ; "Save 1st 16-bit number in DE"  
LHLD 2503H   ; "Get 2nd 16-bit number in H-L pair"  
MOV A, E     ; "Get lower byte of the 1st number"  
ADD L        ; "Add lower byte of the 2nd number"  
MOV L, A     ; "Store result in L-register"  
MOV A, D     ; "Get higher byte of the 1st number"  
ADC H        ; "Add higher byte of the 2nd number with CARRY"  
MOV H, A     ; "Store result in H-register"  
SHLD 4004H   ; "Store 16-bit result in memory locations 2505H and 2506H"  
HLT          ; "Stop"   