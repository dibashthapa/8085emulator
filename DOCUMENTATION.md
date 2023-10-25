
## Table of Instructions

| Instruction | Description | Example |
|-------------|-------------|---------|
| ADD | Add the contents of a register or memory to the accumulator | `ADD B` |
| SUB | Subtract the contents of a register or memory from the accumulator | `SUB M` |
| ADI | Add immediate data to the accumulator | `ADI 32H` |
| MOV | Move the contents of a source register or memory to a destination register | `MOV A, B` |
| MVI | Move immediate data to the specified register or memory | `MVI A, 32H` |
| STA | Store the contents of the accumulator in the specified memory address | `STA 2050H` |
| LDA | Load the contents of the specified memory address into the accumulator | `LDA 2050H` |
| LXI | Load immediate data into the specified register pair | `LXI H, 2050H` |
| DCR | Decrement the contents of the specified register or memory by 1 | `DCR C` |
| INX | Increment the contents of the specified register pair by 1 | `INX D` |
| XCHG | Exchange the contents of the HL and DE register pairs | `XCHG` |
| INR | Increment the contents of the specified register or memory by 1 | `INR M` |