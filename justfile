build TARGET FLAGS MODE:
    cargo b --target {{TARGET}} {{FLAGS}} --profile={{MODE}}

build-rv32e FLAGS MODE: (build "riscv32e-unknown-none-elf" FLAGS MODE)

run TARGET MODE ARCH CPU FLAGS: (build TARGET FLAGS MODE)
    qemu-system-{{ARCH}} -cpu {{CPU}} -bios none -machine virt -serial mon:stdio -nographic -kernel target/{{TARGET}}/{{MODE}}/lightning

run-rv32e MODE CPU FLAGS: (run "riscv32e-unknown-none-elf" MODE "riscv32" CPU FLAGS)

run-rv32e-bare MODE: (run-rv32e MODE "rv32e" "--no-default-features")

run-rv32e-bare-small: (run-rv32e-bare "release-small")

run-rv32e-bare-fast: (run-rv32e-bare "release-fast")
