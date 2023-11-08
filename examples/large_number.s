; The first number 98H is placed in the memory location 2501 H.
; The second number 87H is placed in the memory location 2502H.
; The result is stored in the memory location 2503 H.

LXI H, 2501H ; "Address of first number in H-L pair"
MOV A, M	 ; "1stt number in accumulator"
INX H	     ; "Address of 2nd number in H-L pair"
CMP M	     ; "compare 2nd number with 1st number"
JNC AHEAD	 ; "No, larger is in accumulator. Go to AHEAD"
MOV A, M	 ; "Yes, get 2nd number in the accumulator"
STA 2503 H	 ; "Store larger number in 2503H"
HLT	         ; "Stop"
