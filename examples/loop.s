LXI H,2050H
MVI B,01H
MVI C,0AH
X: MOV M,B
INX H
INR B 
DCR C 
JNZ X
HLT
