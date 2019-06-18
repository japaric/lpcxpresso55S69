crate=lpc55s6x

arm-none-eabi-as -march=armv8-m.main asm.s -o bin/$crate.o
ar crs bin/thumbv8m.main-none-eabi.a bin/$crate.o

rm bin/*.o
