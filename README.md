# Embassy + ESP32

Since there isn't any solid tutorial/example repo on how to get started with
ESP32 and Embassy, let this repo serve you as a guiding light 🔦

## Setting up on WSL

0. Connect your ESP32 with your computer.

1. Install [`usbipd`](https://github.com/dorssel/usbipd-win) on Windows

   ```pwsh
   winget install usbipd
   ```

2. Get the Bus ID of ESP32

   ```pwsh
   usbipd list

   # Connected:
   # BUSID  VID:PID    DEVICE                                                        STATE
   # 2-1    303a:1001  USB Serial Device (COM3), USB JTAG/serial debug unit          Attached
   # 6-1    0489:e0d8  RZ616 Bluetooth(R) Adapter                                    Not shared
   # 6-2    048d:c103  USB Input Device                                              Not shared
   # 6-3    048d:c985  USB Input Device                                              Not shared
   ```

   Here, the device with Bus ID `2-1` is our device. Your Bus ID may vary.

3. Share the device

   Execute this in Powershell with admin privileges:

   ```pwsh
   usbipd bind --busid=<BUSID>
   ```

   Replace `<BUSID>` with the BUSID you found earlier.

4. Attach device to WSL

   ```
   usbipd attach --wsl --busid=<BUSID>
   ```

   Replace `<BUSID>` with the BUSID you found earlier.

5. Add yourself to the `dialout` user group:

   ```sh
   sudo usermod -a -G dialout $USER
   ```

6. Run the program:

   ```sh
   cargo r -r
   ```

Refer to [`usbipd-win`'s](https://github.com/dorssel/usbipd-win) repo for more information.
