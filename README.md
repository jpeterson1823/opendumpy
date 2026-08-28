# OpenDumpy: GB/C Dumping Tool

## What is this?
OpenDumpy is an open-source Gameboy and Gameboy Color cartridge dumper.

Planned Features:
- LCD Display Screen
- MicroSD Card Support
- Capacitive Touch Controls
- ROM Dump
  - Save direct to computer over USB via use of software
  - Save to MicroSD card through on-board UI
- SRAM Dump
  - Save direct to computer over USB via use of software
  - Save to MicroSD card through on-board UI
- SRAM Uploading from file
  - Direct from computer over USB via use of software
  - Load from MicroSD card through on-board UI
- Lots of docs!

## Current Fabrication Status Status

OpenDumpy v3 has finished design and is being checked thoroughly for any design errors.
Ordering of components and PCBs will be completed early September 2026.

OpenDumpy v3 will include development pinouts accessible via IDC connectors on the right side of the board in the event of an uncaught error in PCB/Schematic design. This will allow for the continued development of firmware without needing to have a fixed board this time around.

## Setup & Building the UF2

### Install Required Packages

For Arch Linux:
- `cmake`
- `python`
- `base-devel`
- `arm-none-eabi-gcc`
- `arm-none-eabi-newlib`
- `arm-none-eabi-binutils`

For apt-based systems:
- `cmake`
- `python3`
- `build-essential`
- `gcc-arm-none-eabi`
- `libnewlib-arm-none-eabi`
- `libstdc++-arm-none-eabi-newlib`

### Cloning the Repo

When cloning this repo, make sure to _recursively_ initialize all submodules:

```
git clone https://github.com/jpeterson1823/opendumpy.git
cd opendumpy
git submodule update --init --recursive
```

### Compiling the Firmware

Assuming all package dependencies have been installed, building should be completed with CMake and Make.

It is recommended to create a build folder as to not muddy up the file tree:

```
mkdir firmware/build
cd firmware/build
cmake ..
make -j4
```

### Uploading the Firmware

Press the BOOTSEL button on OpenDumpy, then plug into USB port of machine. It will mount as a USB drive. Upload the built `opendumpy.uf2` file to the mounted drive. Voila!

## Images

### OpenDumpy v3
<div>
<img src=docs/imgs/v3/schematic.png width=100%>
<img src=docs/imgs/v3/footprint.png width=100%>
<img src=docs/imgs/v3/pcb-front.png width=100%>
<img src=docs/imgs/v3/pcb-back.png width=100%>
</div>


### OpenDumpy v1-2
<div>
<img src=docs/imgs/v1-2/opendumpy-v1-render.png width=50%>
<img src=docs/imgs/v1-2/opendumpy-v1-footprint.png width=50%>
<img src=docs/imgs/v1-2/opendumpy-v1-schematic.png width=50%>
</div>
