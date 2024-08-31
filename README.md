# Radio

Utilities, and daemon for interfacing with the Shannon baseband. The following is an overview of the stack:

```mermaid
%%{init: {"flowchart": {"htmlLabels": false}} }%%
flowchart
subgraph "Cellular Processor (CP)"
    shannon("Shannon baseband")
end
subgraph "Application Processor (AP)"
    subgraph "Kernel space"
        linux("Linux kernel")
        dtb("Device Tree Blobs (DTB)")
        driver("Modem v1 (drivers/misc/modem_v1)")
    end
    subgraph "User space"
        subgraph "Universal Mobile Telecommunications Service (UMTS)"
            umts_boot["/dev/umts_boot0"]
            umts_ipc["/dev/umts_ipc0"]
            umts_rfs["/dev/umts_rfs0"]
        end
        subgraph "New Radio (NR)"
            nr_boot["/dev/nr_boot0"]
            nr_boot_spi["/dev/modem_boot_spi"]
            nr_ipc["/dev/nr_ipc0"]
            nr_rfs["/dev/nr_rfs0"]
        end
        subgraph "Programs"
            radiod["Radio daemon (radiod)"]
            interface["Interface with radiod"]
        end
    end
end
shannon <--> linux
linux <--> driver
dtb --> driver
driver <--> umts_boot & umts_ipc & umts_rfs & nr_boot & nr_boot_spi & nr_ipc & nr_rfs
umts_boot & umts_ipc & umts_rfs <--> radiod
nr_boot & nr_ipc & nr_rfs <--> radiod
radiod --> nr_boot_spi
interface <--> radiod
```

Despite the naming, UMTS devices are responsible for 2G GSM, 3G UMTS, 4G LTE, and NR is only 5G NR.

Base structure overview:

```mermaid
classDiagram
note for ToC "Firmware"
class ToC["Table of Contets (ToC) - 32 bytes"] {
    +[u8; 12] label - Name of this entry
    +u32 offset - Offset into the file
    +u32 address - Relative memory address
    +u32 len - Length of the binary
    +u32 crc - CRC checksum of the binary
    +u32 misc - Count of entries if label is ToC
}
note for IpcMessage "IPC"
class IpcMessage["IPC message - 7 bytes"] {
    +u16 len - Length of the message
    +u8 message_sequence - Current message sequence number
    +u8 acknowledge_sequence - Current acknowledgement sequence number
    +u8 category - Functional category
    +u8 which - Which command in the category
    +u8 kind - Discriminantion of the command
    +Vec~u8~ data - Command specific data
}
```

# Links

- [How to boot the Samsung Galaxy S7 modem with plain Linux kernel interfaces only](https://eighty-twenty.org/2020/09/10/booting-samsung-galaxy-s7-modem)
  - Demonstrates the Shannon310 boot process (`/dev/umts_boot0`), and describes the ToC format.
- [LineageOS Exynos9820 kernel sources](https://github.com/LineageOS/android_kernel_samsung_exynos9820/tree/lineage-21/drivers/misc/modem_v1)
  - The Modem v1 driver.
- [Reversing & Emulating Samsung's Shannon Baseband](https://hardwear.io/netherlands-2020/presentation/samsung-baseband-hardwear-io-nl-2020.pdf)
  - Demonstrates various details of the Shannon baseband, and describes the ToC format.
- [Replicant's libsamsung-ipc](https://redmine.replicant.us/projects/replicant/wiki/Libsamsung-ipc)
  - Definitions of various IPC commands, full RIL stack for older devices.
