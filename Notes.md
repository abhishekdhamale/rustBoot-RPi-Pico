

## Things to remember for addding RPi-Pico in rustBoot.

#### Include BOOT2 header in bootloader i.e. Enable BOOT2 header only in bootloader's memory.x.
```
MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
EXTERN(BOOT2_FIRMWARE)
SECTIONS {
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
```

#### Do not include BOOT2 header in firmwares i.e. Disable BOOT2 header in boot firmware and update firmware's memory.x
```
MEMORY {
    FLASH : ORIGIN = 0x10020100, LENGTH = 2048K
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
```
#### While passing address to rom_data::flash_range_* () functions make sure you are passing only offset address

#### While using dereference make sure you use address with FLASH_XIP_BASE = 0x1000_0000

#### Add delay before using rom_data::flash_range_* () functions
