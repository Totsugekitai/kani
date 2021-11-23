.code64
.text

.global x64_switch_context
x64_switch_context:     # fn x64_switch_context(next_ctx: *const u64, current_ctx: *const u64);
    # ===== save context =====
    # save general registers
    mov     [rsi + 0x40], rax
    mov     [rsi + 0x48], rbx
    mov     [rsi + 0x50], rcx
    mov     [rsi + 0x58], rdx
    mov     [rsi + 0x60], rdi
    mov     [rsi + 0x68], rsi
    mov     [rsi + 0x80], r8
    mov     [rsi + 0x88], r9
    mov     [rsi + 0x90], r10
    mov     [rsi + 0x98], r11
    mov     [rsi + 0xa0], r12
    mov     [rsi + 0xa8], r13
    mov     [rsi + 0xb0], r14
    mov     [rsi + 0xb8], r15
    # save stack pointer and base pointer
    lea     rax, [rsp + 8]
    mov     [rsi + 0x70], rax   # rsp
    mov     [rsi + 0x78], rbp
    # save cr3, rip, and rflags
    mov     rax, cr3
    mov     [rsi + 0x00], rax   # cr3
    mov     rax, [rsp]
    mov     [rsi + 0x08], rax   # rip
    pushfq
    pop qword ptr   [rsi + 0x10]    # rflags
    # save segment registers
    mov     ax, cs
    mov     [rsi + 0x20], rax
    mov     ax, ss
    mov     [rsi + 0x28], rax
    mov     ax, fs
    mov     [rsi + 0x30], rax
    mov     ax, gs
    mov     [rsi + 0x38], rax
    # save xmm registers
    fxsave  [rsi + 0xc0]
    # prepare stack frame for iretq
    push qword ptr  [rdi + 0x28]    # ss
    push qword ptr  [rdi + 0x70]    # rsp
    push qword ptr  [rdi + 0x10]    # rflags
    push qword ptr  [rdi + 0x20]    # cs
    push qword ptr  [rdi + 0x08]    # rip

    # ===== load context =====
    # load xmm registers
    fxrstor [rdi + 0xc0]
    # load cr3, rip, and rflags
    mov     rax, [rdi + 0x00]
    mov     cr3, rax
    mov     rax, [rdi + 0x30]
    mov     fs, ax
    mov     rax, [rdi + 0x38]
    mov     gs, ax
    # load general registers
    mov     rax, [rdi + 0x40]
    mov     rbx, [rdi + 0x48]
    mov     rcx, [rdi + 0x50]
    mov     rdx, [rdi + 0x58]
    mov     rsi, [rdi + 0x68]
    mov     rbp, [rdi + 0x78]
    mov     r8, [rdi + 0x80]
    mov     r9, [rdi + 0x88]
    mov     r10, [rdi + 0x90]
    mov     r11, [rdi + 0x98]
    mov     r12, [rdi + 0xa0]
    mov     r13, [rdi + 0xa8]
    mov     r14, [rdi + 0xb0]
    mov     r15, [rdi + 0xb8]
    mov     rdi, [rdi + 0x60]
    # return
    iretq
