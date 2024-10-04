# Emulating the computer's instruction set in software.  

Constructing a computer's cpu with code. This code implements a system inspired by 
CHIP-8 system. Here, we are building a system akin to that which executes the machine code that 
a program consists of.  

##### Short glossary:  
Instruction: This is a primitive CPU command which can be used in moving data, 
working with memory and primitive arithmetic operations e.t.c. Each CPU has its own instruction sets, i.e 
x86, ARM, MIPs, PowerPc, AlphaXP.  


Machine code: This is the code that is being directly processed by the CPU.  


Assembly language: Mnemonic code and some extensions that can easily be understood by humans and 
used to instruct computers.  


CPU registers: These are storage buckets or analogous to untyped temporary variables used by the 
CPU to hold data. Executing programs uses general  purpose register(GPR) file and Application program status 
register. GPRs include R0-R3 and R12(IP) as the volatile registers, R4-R10, R11(FP), R13(SP), R14(LR) as the non-valatile 
registers and PC(R15) which is the program counter.  


Operation: These are procedures supported natively by the system.  

Opcode: This is a number that maps to an operation. Here, they can include both the operation and the operands' registers.  
