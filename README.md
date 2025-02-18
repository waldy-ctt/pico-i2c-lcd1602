# pico-from-scratch
raspberry pi pico rp2040-hal template for rust that i built from sratch hehe

## Detail information about the part
# might update later if i found something that i want to put in hehe

### The Micro Controller
https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf
https://datasheets.raspberrypi.com/rp2040/hardware-design-with-rp2040.pdf
https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf

yeah, really should checkout why the chip called RP2040 in the 3rd link because it really bring in the library we use to control the micro controller.

#### Detail information about the chip
Yeah, like with the other controller, IDK but with this raspberry pi pico. I suggest you to go up to the raspberry page, get information about the controller. Like model, flash memory, ram memory, address,...

for this raspberry pi pico, it use Cortex M0+ processor. So that i would use thumbv6m-none-eabi binary structure. More information you can search up in https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/
since we use the cortex-m processor. Why need to use Cortex-m and Cortex-m-rt crates that provide needed thing to control the Cortex processor.

We also gonna control the micro controller via the HAL layer. And rust having rp-hal crates to control the raspberry pi controller. Since we using the RP2040 raspberry pi pico so that we use the RP2040-hal crate.

We also need a thing to handling the panic of the controller when thing going wrong with the controller. So we also gonna use the panic-halt crate.

Last but not least, using embedded-hal to let the rust to controller the GPIO Pins of the controller.

#### Project structure
For the cortex-m-rt to work, it require us to have a memory.x file that assign the address and capacity of the memory of the controller that usually could be found in the controller's pdf docs.
And also, we also need to config the cargo so it know where, and how the build work by adding file in .cargo/config

#### Text Editor
With the other IDE / TE. IDK but when i was using vscode with the rust analyzer. I had to add another file in .vscode/settings.json to tell the rust-analyzer in vscode that, the target im running gonna be the pico with thumbv6m-none-eabi structure, not our host machine.

```
{
    "rust-analyzer.check.allTargets": false,
    "rust-analyzer.cargo.targets": "thumbv6m-none-eabi"
}
```
#### Pushing the code to pico
at this, we would use a tool named `elf2uf2-rs`.
and by adding this
```
runner = "elf2uf2-rs -d"
```
into the .cargo/config. We would make it auto push the code execute to the mounted pico when it done running.