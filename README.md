# Zeno

A simple x86_64 Operating System Kernel Written in Rust 🦀

Dynamic Memory Management system
- GDT (Global Descriptor Table) to allow running multiple applications without them interfering with each other’s memory space.
- Paging with L4 Page Tables
- Dynamic memory allocation using Frame Allocators and Memory Mappers

Implement Async CPU operations
- IDT (Interrupt Descriptor Table) to execute async operations
- Async Page Fault Handlers
- Timer Interrupt Handlers using PICS
- Key-press Interrupt handlers


Implement I/O
- VGA graphics with a 16-bit color range
- Keyboard drivers with a US 104 Key Layout
- Serial port drivers
