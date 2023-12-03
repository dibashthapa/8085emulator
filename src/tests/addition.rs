#[test]
fn test_add_8_bits() {
    let source = r#"
    LXI H, 2501H  ; "Get address of first number in H-L pair. Now H-L points to 2501H"  
    MOV A, M      ; "Get first operand in accumulator"  
    INX H         ; "Increment content of H-L pair. Now, H-L points 2502H"  
    ADD M         ; "Add first and second operand"  
    INX H         ; "H-L points 4002H"  
    MOV M, A      ; "Store result at 2503H"  
    HLT           ; "Stop"  
    "#;
    let (mut cpu, assembled_count) = crate::execute_code(source);
    cpu.write_memory(0x2501, 0x99);
    cpu.write_memory(0x2502, 0x39);

    while let Some(pc) = cpu.eval() {
        if pc as usize >= assembled_count {
            break;
        }
    }
    assert_eq!(cpu.accumulator, 0xD2);
    assert_eq!(cpu.read_memory(0x2503), 0xD2);
}

#[test]
fn test_add_16_bits() {
    let source = r#"
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
    SHLD 2505; "Store 16-bit result in memory locations 2505H and 2506H"  
    HLT          ; "Stop"    
    "#;

    let (mut cpu, assembled_count) = crate::execute_code(source);
    cpu.write_memory(0x2501, 0x15);
    cpu.write_memory(0x2502, 0x1C);
    cpu.write_memory(0x2503, 0xB7);
    cpu.write_memory(0x2504, 0x5A);

    while let Some(pc) = cpu.eval() {
        if pc as usize >= assembled_count {
            break;
        }
    }
    assert_eq!(cpu.read_memory(0x2505), 0xCC);
    assert_eq!(cpu.read_memory(0x2506), 0x76);
}
