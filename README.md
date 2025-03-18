# RBVM

Register-based virtual machine (RBVM) implemented in Rust.

## Opcode specification

RBVM instructions are built as the following specification describes:
| 8 bits 	| 8 bits   	| 8 bits   	| 8 bits   	|
|---    	|---    	|---    	|---    	|
| Opcode 	| Pad      	| Pad      	| Pad      	|
| Opcode 	| Register 	| Pad      	| Pad      	|
| Opcode 	| Register 	| Integer  	| Pad      	|
| Opcode 	| Register 	| Register 	| Pad      	|
| Opcode 	| Register 	| Register 	| Register 	|
