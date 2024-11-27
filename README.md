## 安装`GStreamer

### ubuntu
```shell
$ sudo apt install build-essential
$ sudo apt install libunwind-dev  # gstreamer 依赖项
$ sudo apt install libgstreamer1.0-dev \
                   libgstreamer-plugins-base1.0-dev \
                   libgstreamer-plugins-bad1.0-dev \
                   gstreamer1.0-plugins-base \
                   gstreamer1.0-plugins-good \
                   gstreamer1.0-plugins-bad \
                   gstreamer1.0-plugins-ugly \
                   gstreamer1.0-libav \
                   gstreamer1.0-tools \
                   gstreamer1.0-x \
                   gstreamer1.0-alsa \
                   gstreamer1.0-gl \
                   gstreamer1.0-gtk3 \
                   gstreamer1.0-qt5 \
                   gstreamer1.0-pulseaudio

$ export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:${PKG_CONFIG_PATH}
```

#### FAQ
- 报错，执行：`pkg-config --cflags gstreamer-1.0`，将依赖项安装好
    ```shell
            pkg-config exited with status code 1
        > PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig/:/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig: PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags gstreamer-1.0 gstreamer-1.0 >= 1.14

        The system library `gstreamer-1.0` required by crate `gstreamer-sys` was not found.
        The file `gstreamer-1.0.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
        PKG_CONFIG_PATH contains the following:
            - /usr/lib/x86_64-linux-gnu/pkgconfig/
            - /usr/lib/x86_64-linux-gnu/pkgconfig
            - /usr/lib/x86_64-linux-gnu/pkgconfig
            - 

        HINT: you may need to install a package such as gstreamer-1.0, gstreamer-1.0-dev or gstreamer-1.0-devel.

        warning: build failed, waiting for other jobs to finish...
    ```