.global rotate_pixels
rotate_pixels:
    // rdi - source pointer  (u8)
    // rsi - target pointer  (u8)
    // rdx - matrix pointer  (f32)
    // rcx - source length

    mov r8, rcx
    xor rcx, rcx

    movups xmm2, [rdx]              // xmm1 [0_f32, 1_f32, 2_f32, null_f32]
    pshufd xmm2, xmm2, 0x6C

    movups xmm3, [rdx + 12]         // xmm2 [3_f32, 4_f32, 5_f32, null_f32]
    pshufd xmm2, xmm2, 0x6C

    movups xmm4, [rdx + 20]         // xmm3 [6_f32, 7_f32, 8_f32, null_f32]
    psrldq xmm4, 4              

loop_start:
    cmp rcx, r8
    je loop_end
    
    xor eax, eax
    movzx eax, byte ptr [rdi + rcx + 2]
    shl eax, 16
    mov ax, word ptr [rdi + rcx]

    movd xmm1, eax
    pmovzxbd xmm1, xmm1
    cvtdq2ps xmm1, xmm1             // xmm1 [0_f32, 1_f32, 2_f32, null_f32]             


    movaps xmm0, xmm1
    mulps xmm0, xmm2
    haddps xmm0, xmm0
    haddps xmm0, xmm0
    cvttss2si eax, xmm0

    // clamp (0 - 255)
    mov ebx, 255
    cmp eax, ebx
    cmovg eax, ebx
    test eax, eax
    mov ebx, 0
    cmovl eax, ebx

    mov byte ptr [rsi + rcx], al
    inc rcx


    movaps xmm0, xmm1
    mulps xmm0, xmm3
    haddps xmm0, xmm0
    haddps xmm0, xmm0
    cvttss2si eax, xmm0

    // clamp (0 - 255)
    mov ebx, 255
    cmp eax, ebx
    cmovg eax, ebx
    test eax, eax
    mov ebx, 0
    cmovl eax, ebx

    mov byte ptr [rsi + rcx], al
    inc rcx


    movaps xmm0, xmm1
    mulps xmm0, xmm4
    haddps xmm0, xmm0
    haddps xmm0, xmm0
    cvttss2si eax, xmm0

    // clamp (0 - 255)
    mov ebx, 255
    cmp eax, ebx
    cmovg eax, ebx
    test eax, eax
    mov ebx, 0
    cmovl eax, ebx

    mov byte ptr [rsi + rcx], al
    inc rcx


    jmp loop_start
    
loop_end:
    ret