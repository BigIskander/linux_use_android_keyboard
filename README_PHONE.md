# linux_use_android_keyboard

[Main Readme](README.md)

Instruction, how to input text from the virtual keyboard of physical device (mobile phone) working on Android (instead of Waydroid).

## How to install and set up
1. Install dependencies:
    ```
    sudo apt install xdotool
    sudo apt install adb
    ```
2. Copy files from `./shell_phone` folder into `~/bin` folder.
3. Copy file `linuxwritetext` into `~/bin` folder. This binary file can be found in releases page or you can get it by compiling source code from `linuxwritetext` folder. You may need to allow execute files as program in file properties.
4. Install `sendtextadb` app in Android phone (binary file `app-debug.apk`, this file can be found in releases page or you can get this file by compiling source code from `sendtextadb` folder).

## How to use
1. In Android phone turn on developer mode and enable USB-debugging.
2. Connect phone to computer by USB cable.
3. Launch linuxwritetext application:
    ```
    linuxwritetext
    ```
4. In the phone launch `sendtextadb` application.
5. Click where text should be entered (text cursor shoud blink).
6. Then, in phone, in `sendtextadb` app write the text (you want to input) by using Android virtual keyboard and then press `send text` button.

Then, programm itself will type the text from your input.
