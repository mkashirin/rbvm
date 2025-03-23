<h1 align="center">RBVM</h1>

<p align=center>Register-based virtual machine (RBVM) implemented in Rust.</p>

## Usage

To start up a REPL execute the following:
```shell
rbvm repl
```
To run a file with a program:
```shell
rbvm run <FILE>
```

## Opcode specification

RBVM instructions are built as the following specification describes:
```
+---------------------------------------------------+
| 32 bits ->                                        |
|---------------------------------------------------|
| Opcode     | 24-bit pad ->                        |
| Opcode     | Register   | 16-bit pad ->           |
| Opcode     | Register   | Register   | 8-bit pad  |
| Opcode     | Register   | Integer    | 8-bit pad  |
| Opcode     | Register   | Register   | Register   |
+---------------------------------------------------+
```
