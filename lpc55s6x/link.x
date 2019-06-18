INCLUDE memory.x

ENTRY(_start_0);

EXTERN(VECTORS_0);
EXTERN(VECTORS_1);
EXTERN(_start_1);

_stack_start_0 = ORIGIN(SRAM0) + LENGTH(SRAM0);
_stack_start_1 = ORIGIN(SRAM1) + LENGTH(SRAM1);

SECTIONS
{
  .vectors ORIGIN(FLASH) :
  {
    LONG(_stack_start_0);
    LONG(_start_0);
    KEEP(*(.vectors.0));

    /* 60 interrupts + 16 exceptions = 76 entries => 128 WORD alignment is required */
    . = ALIGN(4*128);
    _vectors_1 = .;
    LONG(_stack_start_1);
    LONG(_start_1);
    KEEP(*(.vectors.1));
  } > FLASH

  .text : ALIGN(4)
  {
    *(.text .text.*);
  } > FLASH

  .rodata : ALIGN(4)
  {
    *(.rodata .rodata.*);

    . = ALIGN(4);
  } > FLASH

  .bss : ALIGN(4)
  {
    *(.bss .bss.*);

    . = ALIGN(4);
  } > SRAM2

  _sbss = ADDR(.bss);
  _ebss = ADDR(.bss) + SIZEOF(.bss);

  .data : ALIGN(4)
  {
    *(.data .data.*);

    . = ALIGN(4);
  } > SRAM2 AT > FLASH

  _sdata = ADDR(.data);
  _edata = ADDR(.data) + SIZEOF(.data);
  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

ASSERT(_vectors_1 % 128 == 0, "core #1 vector table must be 128-byte aligned");

PROVIDE(NonMaskableInt_0 = DefaultHandler);
PROVIDE(HardFault_0 = DefaultHandler);
PROVIDE(MemoryManagement_0 = DefaultHandler);
PROVIDE(BusFault_0 = DefaultHandler);
PROVIDE(UsageFault_0 = DefaultHandler);
PROVIDE(SecureFault_0 = DefaultHandler);
PROVIDE(SVCall_0 = DefaultHandler);
PROVIDE(DebugMonitor_0 = DefaultHandler);
PROVIDE(PendSV_0 = DefaultHandler);
PROVIDE(SysTick_0 = DefaultHandler);

PROVIDE(WDT_0 = DefaultHandler);
PROVIDE(SDMA0_0 = DefaultHandler);
PROVIDE(GPIO_GLOBALINT0_0 = DefaultHandler);
PROVIDE(GPIO_GLOBALINT1_0 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ0_0 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ1_0 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ2_0 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ3_0 = DefaultHandler);
PROVIDE(UTICK_0 = DefaultHandler);
PROVIDE(MRT_0 = DefaultHandler);
PROVIDE(CTIMER0_0 = DefaultHandler);
PROVIDE(CTIMER1_0 = DefaultHandler);
PROVIDE(SCT_0 = DefaultHandler);
PROVIDE(CTIMER3_0 = DefaultHandler);
PROVIDE(Flexcomm0_0 = DefaultHandler);
PROVIDE(Flexcomm1_0 = DefaultHandler);
PROVIDE(Flexcomm2_0 = DefaultHandler);
PROVIDE(Flexcomm3_0 = DefaultHandler);
PROVIDE(Flexcomm4_0 = DefaultHandler);
PROVIDE(Flexcomm5_0 = DefaultHandler);
PROVIDE(Flexcomm6_0 = DefaultHandler);
PROVIDE(Flexcomm7_0 = DefaultHandler);
PROVIDE(ADC_0 = DefaultHandler);
PROVIDE(ACMP_0 = DefaultHandler);
PROVIDE(USB0_NEEDCLK_0 = DefaultHandler);
PROVIDE(USB0_0 = DefaultHandler);
PROVIDE(RTC_0 = DefaultHandler);
PROVIDE(MAILBOX_0 = DefaultHandler);
PROVIDE(PIN_INT4_0 = DefaultHandler);
PROVIDE(PIN_INT5_0 = DefaultHandler);
PROVIDE(PIN_INT6_0 = DefaultHandler);
PROVIDE(PIN_INT7_0 = DefaultHandler);
PROVIDE(CTIMER2_0 = DefaultHandler);
PROVIDE(CTIMER4_0 = DefaultHandler);
PROVIDE(OSEVTIMER_0 = DefaultHandler);
PROVIDE(SDIO_0 = DefaultHandler);
PROVIDE(USB1_0 = DefaultHandler);
PROVIDE(USB1_NEEDCLK_0 = DefaultHandler);
PROVIDE(HYPERVISOR_0 = DefaultHandler);
PROVIDE(SGPIO_INT0_IRQ0_0 = DefaultHandler);
PROVIDE(SGPIO_INT0_IRQ1_0 = DefaultHandler);
PROVIDE(PLU_0 = DefaultHandler);
PROVIDE(SEC_VIO_0 = DefaultHandler);
PROVIDE(HASH_0 = DefaultHandler);
PROVIDE(CASPER_0 = DefaultHandler);
PROVIDE(PUF_0 = DefaultHandler);
PROVIDE(PQ_0 = DefaultHandler);
PROVIDE(SDMA1_0 = DefaultHandler);
PROVIDE(HS_SPI_0 = DefaultHandler);

PROVIDE(NonMaskableInt_1 = DefaultHandler);
PROVIDE(HardFault_1 = DefaultHandler);
PROVIDE(MemoryManagement_1 = DefaultHandler);
PROVIDE(BusFault_1 = DefaultHandler);
PROVIDE(UsageFault_1 = DefaultHandler);
PROVIDE(SecureFault_1 = DefaultHandler);
PROVIDE(SVCall_1 = DefaultHandler);
PROVIDE(DebugMonitor_1 = DefaultHandler);
PROVIDE(PendSV_1 = DefaultHandler);
PROVIDE(SysTick_1 = DefaultHandler);

PROVIDE(WDT_1 = DefaultHandler);
PROVIDE(SDMA0_1 = DefaultHandler);
PROVIDE(GPIO_GLOBALINT0_1 = DefaultHandler);
PROVIDE(GPIO_GLOBALINT1_1 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ0_1 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ1_1 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ2_1 = DefaultHandler);
PROVIDE(GPIO_INT0_IRQ3_1 = DefaultHandler);
PROVIDE(UTICK_1 = DefaultHandler);
PROVIDE(MRT_1 = DefaultHandler);
PROVIDE(CTIMER0_1 = DefaultHandler);
PROVIDE(CTIMER1_1 = DefaultHandler);
PROVIDE(SCT_1 = DefaultHandler);
PROVIDE(CTIMER3_1 = DefaultHandler);
PROVIDE(Flexcomm0_1 = DefaultHandler);
PROVIDE(Flexcomm1_1 = DefaultHandler);
PROVIDE(Flexcomm2_1 = DefaultHandler);
PROVIDE(Flexcomm3_1 = DefaultHandler);
PROVIDE(Flexcomm4_1 = DefaultHandler);
PROVIDE(Flexcomm5_1 = DefaultHandler);
PROVIDE(Flexcomm6_1 = DefaultHandler);
PROVIDE(Flexcomm7_1 = DefaultHandler);
PROVIDE(ADC_1 = DefaultHandler);
PROVIDE(ACMP_1 = DefaultHandler);
PROVIDE(USB0_NEEDCLK_1 = DefaultHandler);
PROVIDE(USB0_1 = DefaultHandler);
PROVIDE(RTC_1 = DefaultHandler);
PROVIDE(MAILBOX_1 = DefaultHandler);
PROVIDE(PIN_INT4_1 = DefaultHandler);
PROVIDE(PIN_INT5_1 = DefaultHandler);
PROVIDE(PIN_INT6_1 = DefaultHandler);
PROVIDE(PIN_INT7_1 = DefaultHandler);
PROVIDE(CTIMER2_1 = DefaultHandler);
PROVIDE(CTIMER4_1 = DefaultHandler);
PROVIDE(OSEVTIMER_1 = DefaultHandler);
PROVIDE(SDIO_1 = DefaultHandler);
PROVIDE(USB1_1 = DefaultHandler);
PROVIDE(USB1_NEEDCLK_1 = DefaultHandler);
PROVIDE(HYPERVISOR_1 = DefaultHandler);
PROVIDE(SGPIO_INT0_IRQ0_1 = DefaultHandler);
PROVIDE(SGPIO_INT0_IRQ1_1 = DefaultHandler);
PROVIDE(PLU_1 = DefaultHandler);
PROVIDE(SEC_VIO_1 = DefaultHandler);
PROVIDE(HASH_1 = DefaultHandler);
PROVIDE(CASPER_1 = DefaultHandler);
PROVIDE(PUF_1 = DefaultHandler);
PROVIDE(PQ_1 = DefaultHandler);
PROVIDE(SDMA1_1 = DefaultHandler);
PROVIDE(HS_SPI_1 = DefaultHandler);
