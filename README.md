# RBVM

Register-based virtual machine (RBVM) implemented in Rust.

## Opcode specification

RBVM instructions are built as the following specification describes:
```
| 32 bits ->                                        |
|---------------------------------------------------|
| Opcode     | 24-bit pad ->                        |
| Opcode     | Register   | 16-bit pad ->           |
| Opcode     | Register   | Register   | 8-bit pad  |
| Opcode     | Register   | Integer    | 8-bit pad  |
| Opcode     | Register   | Register   | Register   |
```
