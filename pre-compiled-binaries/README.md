
## Step 1. Flash Update-Firmware to Partition (0x10040000)
	
 ```probe-rs-cli download --format Bin --base-address 0x10040000 --chip RP2040 rp2040_updtfw_v1235_signed.bin```

## Step 2. Flash Boot-Firmware to Partition (0x10020000)
	
 ```probe-rs-cli download --format Bin --base-address 0x10020000 --chip RP2040 rp2040_bootfw_v1234_signed.bin```

## Step 3. Flash rustBoot to Boot Partition (0x10000000)

 ```probe-rs-cli download --format Bin --base-address 0x10000000 --chip RP2040 rp2040.bin```

## Step 4. Reset Power
