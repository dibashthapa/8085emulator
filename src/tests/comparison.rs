#[test]
fn test_large_number() {
    // TODO: There is an issue with comparing with 0x0A value
    let source = r#"
;Find the largest number in a block of data.  
;The length of the block is in memory location 2200H and the block itself starts from memory location 2201H.
;Store the largest number in memory location 2300H. Assume that the numbers in the block are all 8-bit unsigned binary numbers.

LXI H,2200H             ; Set pointer for array.
MOV B, M                ; Load the Count.
INX H                          
MOV A, M                ; Set 1st element as largest data.
DCR B                   ; Decrement the count.
LOOP: INX H        
CMP  M                  ; If A- reg > M go to AHEAD
JNC AHEAD    
MOV A, M                ; Set the new value as largest.
AHEAD: DCR  B         
JNZ LOOP                ; Repeat comparisons till count = 0.
STA 2300H               ; Store the largest value at 2300.
HLT                     ; Terminate program execution.
"#;

    let (mut cpu, assembled_count) = crate::execute_code(source);
    cpu.write_memory(0x2200, 0x04);
    cpu.write_memory(0x2201, 0x01);
    cpu.write_memory(0x2202, 0x05);
    cpu.write_memory(0x2203, 0x07);
    cpu.write_memory(0x2204, 0x08);

    while let Some(pc) = cpu.eval() {
        if pc as usize >= assembled_count {
            break;
        }
    }
    assert_eq!(cpu.read_memory(0x2204), 0x08);
}
