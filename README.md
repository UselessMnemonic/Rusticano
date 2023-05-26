# Rusticano
_A WIP educational Java VM for all devices!_

This is Rusticano, a Java VM written in Rust with the goal of teaching developers (especially
myself) how to design virtual machines with as many (or as few) features as possible.

Rusticano comes from a portmanteau of Rust and Americano, two human inventions that incidentally
have great effects on my productivity. The thought goes that if Rust can target it, then so should
Java!

## Modules
### Java
The `java` module is an implementation of the Java CLI. This can be build on hosts with a shell
or standard library. Other users will need to implement their own frontends using the `jvm` module.

### JVM
The `jvm` module contains the Rusticano core. This can be used in your project to spawn and
manipulate a Java VM.

## Design
Rusticano is designed to be modular, by which individual JVM features of varying memory and
architecture requirements can be segregated and chosen on a per-target basis. 

Rusticano requires at least these items be implemented per target:
 - A System Classloader
 - A Bytecode Interpreter
 - Heap Allocators
 - Converters between Object Handles and Heap Pointers

### System Classloader
The System Classloader is not expected to dynamically load classes. One target may have all its
classes inside program ROM, while another may want to load from a filesystem. To accommodate these
cases, Rusticano makes few assumptions about what capabilities are shipped with the System
Classloader.

### Bytecode Interpreter
Rusticano can theoretically support any subset of Java Opcodes. This allows Rust to optimize
Rusticano for any desired level of complexity. For example, if a target never expects to use
`invokedynamic` or the Reflection API, then the corresponding logic can be omitted from the final
binary.

### Heap Allocators and Handle Converters
Rusticano delegates all raw pointer operations to target-specific Handles. This is required for
supporting different Garbage Collection strategies and pointer encodings. Targets must implement
conversions between Handles and Object References so that Rusticano never manipulates memory until
absolutely necessary.
