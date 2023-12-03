#[test]
fn subtract_8_bits() {
    let source = r#"
;(2501 H) = 49H 
;(2502 H) = 32H 
;Result (2503 H) = 49H - 32H = 17H  
LXI H, 2501H   ;  "Get address of first number in H-L pair. Now H-L points to 2501H"  
MOV A, M       ;  "Get first operand in accumulator"  
INX H          ;  "Increment content of H-L pair. Now, H-L points 2502H"  
SUB M          ;  "Subtract first to second operand"  
INX H          ;  "H-L points 4002H"  
MOV M, A       ;  "Store result at 2503H"  
HLT            ;  "Stop"  
 "#;
    let (mut cpu, assembled_count) = crate::execute_code(source);
    cpu.write_memory(0x2501, 0x49);
    cpu.write_memory(0x2502, 0x32);

    while let Some(pc) = cpu.eval() {
        if pc as usize >= assembled_count {
            break;
        }
    }
    assert_eq!(cpu.read_memory(0x2503), 0x17);
}
