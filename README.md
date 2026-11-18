# Blinky STM32F

This is a little "Hello, world!" style example for the STM32F0 discovery board in Rust.

It's mostly intended to demonstrate what a minimal setup for embedded development in Rust can look like.

To make it more interesting than the usual on/off blinky LED, it adds the following:

- SysTick at 1 ms intervals to demonstrate exception handling, with interrupts being similar
- PWM at ~1 kHz to fade the LEDs in and out
- Static mutable state in the SysTick handler to track the phase of the animation

Please note that nothing about this is necessarily "best practice". There are probably linker settings that could be improved, and there's all sorts of embedded stuff and Rust best practices that I'm likely ignoring (powering down the core to save energy, using an interrupt-free critical section in the SysTick ISR doesn't seem ideal, so on). It's just meant as a starting point for getting code running on a device.
