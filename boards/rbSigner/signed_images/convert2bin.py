import os

firmware = ["rp2040_bootfw", "rp2040_updtfw"]
target_path = "../../target/thumbv6m-none-eabi/release/"


def convert_to_bin(path):
    for filename in os.listdir(path):
        if filename == "rp2040_bootfw" and (filename + ".bin") not in os.listdir(os.getcwd()):
            os.system("rust-objcopy --strip-all -I elf32-littlearm" + " " + target_path +
                      filename + " " + "-O binary rp2040_bootfw.bin")
        elif filename == "rp2040_updtfw" and (filename + ".bin") not in os.listdir(os.getcwd()):
            os.system("rust-objcopy --strip-all -I elf32-littlearm" + " " + target_path +
                      filename + " " + "-O binary rp2040_updtfw.bin")
convert_to_bin(target_path)


