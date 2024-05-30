# linux_use_android_keyboard

[In english](README.md)

Способ испльзовать виртуальную клавиатуру Андроида в Linux.

Видео демонстрация:

https://github.com/BigIskander/linux_use_android_keyboard/assets/10581861/a61e21ab-0c4d-4f33-b6e1-cde9f7153a98

Программы разрабатывались и тестировались в Linux с окружением рабочего стола X11.

## Как установить и настроить

1. Установить зависимости:
    ```
    sudo apt install xdotool
    sudo apt install weston
    ```
2. Установить Waydroid по инструкции из официального сайта [Waydroid](https://waydro.id/).
3. Скопировать файлы из папки `./shell` в папку `~/bin`.
4. Скопировать файл `linuxwritetext` в папку `~/bin`. Этот бинарный файл можно найти на странице релизов или получить скомпилировав исходный код программы из папки 'lunuxwritetext'. Возможно, понадобится разрешить выполнять как программу в свойствах файлов.
5. Запустить Waydroid:
    ```
    waydroid-run
    ```
    Установить в Waydroid приложение для Андроид:
    ``` 
    waydroid app install app-debug.apk
    ```
    Бинарный файл `app-debug.apk` можно найти на странице релизов или получить, скомпилировав исходный код программы из папки `sendtextadb`.

## Как использовать

1. Запустить waydroid:
    ```
    waydroid-run
    ```
2. Запустить приложение linuxwritetext:
    ```
    linuxwritetext
    ```
3. Внутри Waydroid запустить приложение `sendtextadb`.
4. Кликнуть по текстовому полю, куда должен быть введен текст (чтобы замигал курсор ввода).
5. Далее в Waydroid в приложении `sendtextadb` написать нужный текст с использованием виртуальной клавиатуры Android и нажать кнопку  `send text`.

Далее, программа сама инициализирует нажатие клавиш alt+Tab (чтобы переключиться в предыдущее активное окно), напечатает тот текст, что вы ввели и потом снова инициализирует alt+Tab (чтобы переключиться обратно в окно Weydroid).

## Примечания 

Я для демонстрации устанавливал Android клавиатуру gboard.

Чтобы не набирать пароль при запуске `linuxwritetext` можно добавить команду `waydroid logcat` в исключения, следующим образом:
1. Выполнить команду:   
    ```
    sudo visudo -f /etc/sudoers.d/waydroid
    ```
2. Вписать:
    ```
    %sudo ALL=NOPASSWD: /usr/bin/waydroid logcat
    ```
    и сохранить.

