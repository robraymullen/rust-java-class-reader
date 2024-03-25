# A Rust based Java Class file reader
Built according to this spec: https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html

Mostly just for self learning. End goal is to have a cmd line app that can pretty print class file contents but in a simpler, less verbose way than javap.

Would also like to add a basic interpreter for the java bytecode itself. This would be a separate library that can do a REPL or evaluation of bytecode. The aim would not be to evaluate a full class file, only opcode instructions. So that the instructions are used separately from a classfile, as if they were a type of assembly.
