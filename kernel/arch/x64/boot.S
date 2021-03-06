.section ".boot", "ax" # このセクションフラグを付けないとisn't in a segmentと出る

.code32

.align 8
multiboot2_header_start:
    .long   0xe85250d6                                      # マジックナンバー
    .long   0                                               # アーキテクチャ(i386)
    .long   multiboot2_header_end - multiboot2_header_start   # multibootヘッダの長さ
    .long   -(0xe85250d6 + 0 + (multiboot2_header_end - multiboot2_header_start))    # チェックサム

multiboot2_inforeq:
    .short  1   # inforeq type
    .short  0   # inforeq flags
    .long   multiboot2_inforeq_end - multiboot2_inforeq
    .long   6   # memory mapのリクエスト
multiboot2_inforeq_end:

.align 8
terminator:
    .short  0
    .short  0
    .long   8
terminator_end:
multiboot2_header_end:

.align 8
multiboot_header:
    .long   0x1badb002
    .long   0x00000000
    .long   -(0x1badb002 + 0x00000000)
multiboot_header_end:

.code32
.global boot
boot:
    cli     # 割り込み禁止
    # スタック切り替え
    mov     esp, offset __kernel_stack

    # multibootの情報を64bitでスタックに保存
    push    0
    push    eax     # マジック
    push    0
    push    ebx     # multibootにリクエストした情報

    # GDTをロードしてretfでDescriptor番号を指定して飛ぶ
    mov     eax, 24         # CSの構築 GDT3番を指定
    push    eax             # CSの内容をスタックに積む
    lea     eax, [boot2]    # 次のジャンプ先を指定
    push    eax             # 次のジャンプ先をスタックに積む
    lgdt    [load_gdt]      # GDTをロード
    retf

.global boot2
boot2:
    # protected modeなのでCS以外のセグメントレジスタを合わせてやる
    mov     ax, 16  # GDT2番
    mov     ds, ax
    mov     es, ax
    mov     fs, ax
    mov     gs, ax
    mov     ss, ax

    # ここから64bitモード移行作業
    # カーネルのページテーブルの構築を行う
    # ここでは2MBページング
.global first_setup_page_tables
first_setup_page_tables:
    lea     eax, [__kernel_pagetable_pdpt]
    or      eax, 0b11
    mov     [__kernel_pagetable_pml4], eax

    lea     eax, [__kernel_pagetable_pd]
    or      eax, 0b11
    mov     [__kernel_pagetable_pdpt], eax

    mov     ecx, 0
.map_p2_table:
    mov     eax, 0x200000
    mul     ecx
    or      eax, 0b10000011
    mov     [__kernel_pagetable_pd + ecx * 8], eax
    inc     ecx
    cmp     ecx, 512
    jne     .map_p2_table

    # 64bitモードに移行
change_to_long_mode:
    # 一旦 CR0.PG = 0 にする
    mov     eax, cr0
    and     eax, 0x7fffffff
    mov     cr0, eax
    # PAEとPGEを1にセット
    mov     eax, cr4
    or      eax, 0x20
    mov     cr4, eax
    # ページテーブルをセット
    lea     eax, [__kernel_pagetable_pml4]
    mov     cr3, eax
    # 64bitモードをon
    mov     ecx, 0xc0000080
    rdmsr
    or      eax, 0x0900
    wrmsr
    # retfでCSを変えて飛ぶための準備
    mov     eax, 8                  # CSの構築(GDT1番)
    push    eax
    lea     eax, [start_long_mode]  # 飛び先の準備
    push    eax
    # ページングをon (CR0.PG = 1, CR0.PE = 1)
    mov     eax, cr0
    or      eax, 0x1
    mov     cr0, eax
    mov     eax, cr0
    or      eax, 1 << 31
    # hlt
    mov     cr0, eax
    retf

.align 16
.global load_gdt
load_gdt:
    .word   gdt_end - gdt - 1
    .quad   gdt

.align 16
gdt:
    .quad   0x0000000000000000  # 先頭はnull descriptor
    .quad   0x00af9a000000ffff  # kernel 64bit code segment
    .quad   0x00cf92000000ffff  # kernel 64bit data segment
    .quad   0x00cf9a000000ffff  # kernel 32bit code segment
gdt_end:

.code64
.text
start_long_mode:
    mov     ax, 0
    mov     ss, ax
    mov     ds, ax
    mov     es, ax
    mov     fs, ax
    mov     gs, ax
second_setup_page_tables:
    # 追加のPDPTの構築
    lea     edi, [__kernel_pagetable_pdpt + 8]
    lea     eax, [__kernel_pagetable_pd + 0x1000 + 0x103] # Present, writable, global
    mov     ecx, 3 # 追加するPDPTエントリの数
.write_pdpt_entry:
    mov dword ptr   [edi], eax
    mov dword ptr   [edi + 4], 0
    add     eax, 0x1000
    add     edi, 8
    loop    .write_pdpt_entry
    # PD
    lea     edi, [__kernel_pagetable_pd + 8 * 512]
    mov     eax, 0x0000183 # Present, writable, global, ページサイズを2MB
    add     eax, 0x200000 * 512
    mov     ecx, 3 * 512 # PDPTエントリ数 * PDエントリ数
.write_pd_entry:
    mov dword ptr   [edi], eax
    mov dword ptr   [edi + 4], 0
    add     eax, 0x200000 # 2MB
    add     edi, 8
    loop    .write_pd_entry

    # .bssセクションをクリア
    xor     al,al
    lea     rdi, [__bss]
    lea     rcx, [__bss_end]
    sub     rcx, rdi
    cld
    rep stosb
    # multibootの情報を取得
    pop     rsi # multiboot infoのアドレス
    pop     rdi # multibootのmagic

    lea     rax, [init_x86]
    call    rax

halt:
    cli
    hlt
    jmp     halt
