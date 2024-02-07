.global rotate_pixels
rotate_pixels:
    // rdi - source pointer  (u8)
    // rsi - target pointer  (u8)
    // rdx - matrix pointer  (f32)
    // rcx - source length

    mov r8, rcx
    xor rcx, rcx

    movups xmm2, [rdx]                  // xmm1 [idx0_f32, idx1_f32, idx2_f32, null_f32]
    pshufd xmm2, xmm2, 0x6C

    movups xmm3, [rdx + 12]             // xmm2 [idx3_f32, idx4_f32, idx5_f32, null_f32]
    pshufd xmm2, xmm2, 0x6C

    movups xmm4, [rdx + 20]             // xmm3 [idx6_f32, idx7_f32, idx8_f32, null_f32]
    psrldq xmm4, 4              

loop_start:
    cmp rcx, r8
    je loop_end
    
    movzx eax, byte ptr [rdi + rcx + 2] // move (B) value to eax
    shl eax, 16
    mov ax, word ptr [rdi + rcx]        // move (R) and (G) value to eax

    movd xmm1, eax
    pmovzxbd xmm1, xmm1                 // spread u8 to u32
    cvtdq2ps xmm1, xmm1                 // ints to floats            


    movaps xmm0, xmm1
    mulps xmm0, xmm2
    haddps xmm0, xmm0               
    haddps xmm0, xmm0                   // sum floats
    cvttss2si eax, xmm0                 // convert floats to ints

    // clamp (0 - 255)
    mov ebx, 255
    cmp eax, ebx
    cmovg eax, ebx
    test eax, eax
    mov ebx, 0
    cmovl eax, ebx

    mov byte ptr [rsi + rcx], al        // move (R) to target


    movaps xmm0, xmm1
    mulps xmm0, xmm3
    haddps xmm0, xmm0
    haddps xmm0, xmm0                   // sum floats
    cvttss2si eax, xmm0                  // convert floats to ints

    // clamp (0 - 255)
    mov ebx, 255
    cmp eax, ebx
    cmovg eax, ebx
    test eax, eax
    mov ebx, 0
    cmovl eax, ebx

    mov byte ptr [rsi + rcx + 1], al    // move (G) to target


    movaps xmm0, xmm1
    mulps xmm0, xmm4
    haddps xmm0, xmm0
    haddps xmm0, xmm0                   // sum floats
    cvttss2si eax, xmm0                  // convert floats to ints


    // clamp (0 - 255)
    mov ebx, 255
    cmp eax, ebx
    cmovg eax, ebx
    test eax, eax
    mov ebx, 0
    cmovl eax, ebx

    mov byte ptr [rsi + rcx + 2], al    // move (B) to target


    add rcx, 3
    jmp loop_start
    
loop_end:
    ret