target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

# To re-enable hprintln!() macro logs add this line back
#monitor arm semihosting enable

# # send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 16000000 must match the core clock frequency
# Followed instructions from here: https://docs.rs/itm/0.2.1/itm/
# Run this with: $ itmdump -f /tmp/itm.fifo -F
# to get these logs
monitor stm32l4x.tpiu disable
monitor stm32l4x.tpiu configure -protocol uart -traceclk 16000000 -pin-freq 2000000 -output /tmp/itm.fifo -formatter off
monitor stm32l4x.tpiu enable

# # enable ITM port 0
monitor itm port 0 on

# Program the application into flash-memory
load

# start the application running
cont
