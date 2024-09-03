# linux_use_android_keyboard

[По русски](README_RUS.md)

A method to use Android virtual keyboard in Linux.

Video demonstration:

https://github.com/BigIskander/linux_use_android_keyboard/assets/10581861/a61e21ab-0c4d-4f33-b6e1-cde9f7153a98

Programs were developed and tested in Linux with X11 desktop environment.

## How to install and set up

1. Install dependencies:
    ```
    sudo apt install xdotool
    sudo apt install weston
    ```
2. Install Waydroid by instruction from the official website [Waydroid](https://waydro.id/).
3. Copy files from `./shell` folder into `~/bin` folder.
4. Copy file `linuxwritetext` into `~/bin` folder. This binary file can be found in releases page or you can get it by compiling source code from `linuxwritetext` folder. You may need to allow execute files as program in file properties.
5. Launch Waydroid:
    ```
    waydroid-run
    ```
    Install Android aplication into Waydroid:
    ``` 
    waydroid app install app-debug.apk
    ```
    Binary file `app-debug.apk` can be found in releases page or you can get this file by compiling source code from `sendtextadb` folder.

## How to use

1. Launch waydroid:
    ```
    waydroid-run
    ```
2. Launch linuxwritetext application:
    ```
    linuxwritetext
    ```
3. Inside Waydroid launch `sendtextadb` application.
4. Click where text should be entered (text cursor shoud blink).
5. Then inside Waydroid in `sendtextadb` write the text by using Android virtual keyboard and click `send text` button.

Then, program itself initialize alt+Tab keypress (in order to switch to previous active window), type the text you typed and then initialize alt+Tab keypress again (in order to switch back to Waydroid window).

**UPDATE**: [Instruction, how to input text from the virtual keyboard of physical device (mobile phone) working on Android (instead of Waydroid).](README_PHONE.md)

## Notes

For the demonstration I used gboard Android keyboard.

In order to not enter password when launch `linuxwritetext` you can add `waydroid logcat` command to exeptions, this way:
1. Execute command:
    ```
    sudo visudo -f /etc/sudoers.d/waydroid
    ```
2. Write:
    ```
    %sudo ALL=NOPASSWD: /usr/bin/waydroid logcat
    ```
    and save.

