.globl start
start:
        la a0, init
        la a1, argv
        li a7, 114
        j inf_loop
inf_loop:
    ecall
    j inf_loop

# for(;;) exit();
exit:
        li a7, 42
        ecall
        jal exit

# char init[] = "/init\0";
init:
  .string "/init\0"

# char *argv[] = { init, 0 };
.p2align 2
argv:
  .long init
  .long 0
