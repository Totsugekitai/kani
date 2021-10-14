.section ".boot"
.intel_syntax noprefix

#define MULTIBOOT2_MAGIC        0xe85250d6
#define MULTIBOOT2_ARCH_I386    0
#define INFOREQ_TYPE            1
#define INFOREQ_FLAGS           0

.align 8
multiboot2_header:
    .long   MULTIBOOT2_MAGIC                            # マジックナンバー
    .long   MULTIBOOT2_ARCH_I386                        # アーキテクチャ(i386)
    .long   multiboot2_header_end - multiboot2_header   # multibootヘッダの長さ
    .long   -(MULTIBOOT2_MAGIC + MULTIBOOT2_ARCH_I386 + (multiboot2_header_end - multiboot2_header))    # チェックサム

multiboot2_inforeq:
    .short  INFOREQ_TYPE
    .short  INFOREQ_FLAGS
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

.align 16
.global load_gdt
load_gdt:
    .word   gdt_end - gdt - 1
    .long   gdt

.align 16
gdt:
    .quad   0x0000000000000000  # 先頭はnull descriptor
    .quad   0x00af9a000000ffff  # kernel 64bit code segment
    .quad   0x00cf92000000ffff  # kernel 64bit data segment
    .quad   0x00cf9a000000ffff  # kernel 32bit code segment
gdt_end:

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
    # PML4の構築
construct_kernel_pml4:
    lea     eax, [__kernel_pagetable_pml4]
    lea     ebx, [__kernel_pagetable_pdpt + 0b11]  # Present, Read/Write
    # PML4のエントリを1つ構築
    mov dword ptr   [eax + 0], ebx
    mov dword ptr   [eax + 4], 0
    # 以下はPML4の2つ目以降のエントリを無効にしておくためにP=0を置いておく処理
    mov     ecx, 1
construct_kernel_pml4_loop:
    mov     edx, ecx
    mul     edx, 8
    add     edx, eax
    mov dword ptr   [edx + 0], 0
    mov dword ptr   [edx + 4], 0
    inc     ecx
    cmp     ecx, 512
    jne     construct_kernel_pml4_loop

    # PDPTの構築
construct_kernel_pdpt:
    lea     eax, [__kernel_pagetable_pdpt]
    lea     ebx, [__kernel_pagetable_pd + 0b11]     # Present, Read/Write
    # PDPTのエントリを1つ構築
    mov dword ptr   [eax + 0], ebx
    mov dword ptr   [eax + 4], 0
    # 以下はPDPTの2つ目以降のエントリを無効にしておくためにP=0を置いておく処理
    mov     ecx, 1
construct_kernel_pdpt_loop:
    mov     edx, ecx
    mul     edx, 8
    add     edx, eax
    mov dword ptr   [edx + 0], 0
    mov dword ptr   [edx + 4], 0
    inc     ecx
    cmp     ecx, 512
    jne     construct_kernel_pdpt_loop

    # PDの構築
construct_kernel_pd:
    lea     eax, [__kernel_pagetable_pd]
    # PDのエントリの0番目(physical address: 0x000000 ~ 0x0fffff は無効にしておく)
    mov dword ptr   [eax + 0], 0
    mov dword ptr   [eax + 4], 0
    # 以下はPDのエントリ登録(2MiB x 63 = 126MiB)
    mov     ecx, 1
    mov     edi, 0x183      # Global, Page size, R/W, Present
construct_kernel_pd_loop_0:
    add     edi, 0x100000
    mov     edx, ecx
    mul     edx, 8
    add     edx, eax
    mov dword ptr   [edx + 0], edi
    mov dword ptr   [edx + 4], 0
    inc     ecx
    cmp     ecx, 64
    jne     construct_kernel_pd_loop_0
    # 以下はPDPTの以降のエントリを無効にしておくためにP=0を置いておく処理
    mov     ecx, 1
construct_kernel_pd_loop_1:
    mov     edx, ecx
    mul     edx, 8
    add     edx, eax
    mov dword ptr   [edx + 0], 0
    mov dword ptr   [edx + 4], 0
    inc     ecx
    cmp     ecx, 512
    jne     construct_kernel_pd_loop_1

    # 64bitモードに移行
change_to_long_mode:
    # PAEとPGEを1にセット
    mov     eax, cr4
    or      eax, 0xa0
    mov     cr4, eax
    # ページテーブルをセット
    lea     eax, [__kernel_pagetable_pml4]
    mov     cr3, eax
    # 64bitモードをon
    mov     ecx, 0xc0000080
    rdmsr
    or      eax, 0x100
    wrmsr
    # retfでCSを変えて飛ぶための準備
    mov     eax, 8                  # CSの構築(GDT1番)
    push    eax
    lea     eax, [start_long_mode]  # 飛び先の準備
    push    eax
    # ページングをon
    mov     eax, cr0
    or      eax, 0x80000000
    mov     cr0, eax
    retf

.code64
start_long_mode:
    