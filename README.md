# GStreamer
### 编译安装
```shell
python3 -m pip install meson
rustup update
cargo install cargo-c

git clone https://gitlab.freedesktop.org/gstreamer/gstreamer.git
cd gstreamer
sudo apt install libnice-dev libnice10
sudo apt install libxkbcommon-dev libxkbcommon0
sudo apt install nasm yasm
sudo apt install libvulkan-dev libvulkan1 libvulkan-volk-dev glslc
librust-cairo-rs-dev librust-cairo-sys-rs-dev 
sudo apt install mono-devel
sudo apt install -y libgtk-3-dev
meson setup build --prefix `pwd`/../gst-install -Drs=enabled -Dvaapi=enabled -Dgtk=enabled
/data/nvme1n1p1/zhangningboo/miniforge3/bin/meson setup build --prefix `pwd`/../gst-install -Drs=enabled -Dvaapi=enabled
/data/nvme1n1p1/zhangningboo/miniforge3/bin/meson setup build --prefix /data/nvme1n1p1/zhangningboo/rust-workspace/gstreamer-tutorial/gst-install -Dvaapi=enabled

ninja -C build devenv
```
### 安装
```shell
$ sudo apt-get update
$ sudo apt-get install gstreamer1.0-tools \
    gstreamer1.0-alsa \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav
$ sudo apt-get install libgstreamer1.0-dev \
     libgstreamer-plugins-base1.0-dev \
     libgstreamer-plugins-good1.0-dev \
     libgstreamer-plugins-bad1.0-dev
$ gst-inspect-1.0 --version
```

### rust环境安装
```shell
$ vim ~/.bashrc
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
$ source ~/.bashrc
$ curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
$ vim ~/.cargo/config
[source.crates-io]
replace-with = 'rsproxy'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true

$ ln -s ~/.cargo/config ~/.cargo/config.toml
```

### c++环境安装
```shell

```

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