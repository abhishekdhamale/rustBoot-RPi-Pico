
import os
import shutil

key_file = "ecc256.der"
firmware = ["rp2040_bootfw.bin", "rp2040_updtfw.bin"]
target_path = "."


def sign_image(path):
    for filename in os.listdir(path):
        if key_file not in os.listdir(os.getcwd()):
            shutil.copy("../keygen/ecc256.der", os.getcwd())
        # boot image - version 1234
        if filename == "rp2040_bootfw.bin":
            # sign `bin` file
            os.system("py sign.py --ecc256 --sha256" + " " +
                      filename + " ecc256.der 1234")
        # update image - version 1235
        elif filename == "rp2040_updtfw.bin":
            # sign `bin` file
            os.system("py sign.py --ecc256 --sha256" + " " +
                      filename + " ecc256.der 1235")
def clean_up():
    os.remove(os.getcwd() + "/ecc256.der")


sign_image(target_path)
clean_up()
