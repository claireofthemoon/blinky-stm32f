target extended-remote :3333
set print asm-demangle on
break DefaultHandler
break HardFault
monitor arm semihosting enable
load
stepi
