import os
import random
import ctypes
from pathlib import Path


WALLPAPER_FOLDER = r"C:\Users\YourName\Pictures\Wallpapers"
IMAGE_FORMATS = {'.jpg', '.jpeg', '.png', '.bmp'}

def change_wallpaper():

    folder = Path(WALLPAPER_FOLDER)

    images = [
        str(file.resolve())
        for file in folder.iterdir()
        if file.is_file() and file.suffix.lower() in IMAGE_FORMATS
    ]

    if not images:
        print("")
        return False


    selected_image = random.choice(images)
    print(f": {selected_image}")


    ctypes.windll.user32.SystemParametersInfoW(20, 0, selected_image, 3)
    return True

if __name__ == "__main__":
    change_wallpaper()
