# Defective Echo Sample

This sample is intended to validate the behavior of the Driver Verifier against a faulty driver written in Rust. 

The sample is similar in design and functionality to the ECHO (DriverSync) sample, the difference being this sample intentionally does not free the memory allocated for the queue buffer.
When an application writes to a device managed by this driver, the driver creates a buffer for the WDFQUEUE (managed in the Queue's Context). Ideally, this memory is freed when there is another write or when the queue context is destroyed. In order to validate the DV's behavior, this sample intentionally does not free any memory allocated using 'ExAllocatePool2' DDI. 

## Steps to Reproduce the issue

1. Clone the repository and navigate to the project directory.

2. Build the driver project using the following command in a WDK environment (or EWDK prompt) - 
    ```
    cargo make
    ```
3. Prepare a target system (a Hyper-V VM can be used) for testing
    Target System OS: Windows 11 Dev Environment 

    Follow the below steps to setup the test system -
    1. Disable Secure boot and start the system
    2. Run "ipconfig" on the host system and note down the IP (if you are using Default Switch for the VM, note down the IP on the Default Switch)
    3. Install and open WinDbg, click on "Attach to Kernel". The key for the connection will be generated in the test system in the next steps. 
    2. Connect to the test VM and run the following commands - 
        ```
        bcdedit /set testsigning on
        bcdedit /debug on
        bcdedit /dbgsettings net hostip:<PASTE.HOST.IP.HERE> port:<50000-50030>

        ### Copy the key string output by the above command
        ```
    4. Paste the key in host's WinDbg prompt and connect to the kernel
    5. Restart the target/test system 
        ```
        shutdown -r -t 0
        ```

4. Copy the driver package, available under ".\target\debug\defective_echo_package" to the target system.
    NOTE: You may modify the CopyDriverPackage.ps1 to copy the driver package to the target VM 

5. Copy "devgen.exe" from host to the target system. Alternatively you may install WDK on the target system and add the directory that contains "devgen.exe" to PATH variable.

6. Install the driver package and create the device in the target system using the below commands - 
    ```
    cd "defective_echo_package"
    devgen.exe /add /bus ROOT /hardwareid "defective_echo"

    ## Copy the Device ID. This will be used later to run the tests

    pnputil.exe /add-driver .\defective_echo.inf /install
    ```
7. Enable Driver Verifier for 'defective_echo.sys' driver package 
    1. Open run command promt (Start + R) or cmd as administator and run "verifier"
    2. In the verifier manager,
        - Create Standard Settings
        - Select driver names from list
        - Select 'defective_echo.sys'
        - Finish
        - Restart the system

8. Follow the steps in https://learn.microsoft.com/en-us/windows-hardware/drivers/develop/how-to-test-a-driver-at-runtime-from-a-command-prompt to run tests against the device managed by this driver

9. Run the following test after TAEF and WDTF are installed -
    ```
    cd "C:\Program Files (x86)\Windows Kits\10\Testing\Tests\Additional Tests\x64\DevFund"
    TE.exe .\Devfund_CHAOS_WLK_Certification.dll /P:"DQ=DeviceID='ROOT\DEVGEN\{A0ED791E-417E-0644-BD2E-85F05BA10567}'" --rebootResumeOption:Manual
    ```

10. WDTF_PNP: EDTSurpriseRemoveDevice() test will fail. The target system shows a Blue Screen Error with the following error - 
    ```
    DRIVER_VERIFIER_DETECTED_VIOLATION (c4)
    ```
    The logs will be available in WinDbg
    run analyze v for detailed information