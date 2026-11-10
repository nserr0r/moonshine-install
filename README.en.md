<div align="center">

# 🌙 moonshine-install

**The evillin installer: partitions the disk, encrypts it, installs packages from repositories and sets up booting.**

🌐 [Русский](README.md) · **English**

</div>

---

## 🌙 What it is

`moonshine-install` installs the system onto a clean disk. There are three ways to use it:

| Way | For | Command |
|---|---|---|
| **Wizard** | a first install: questions on screen, everything picked from lists | `moonshine-install` |
| **Config** | identical and unattended installs: everything in one file | `moonshine-install --config moonshine.toml` |
| **Flags** | scripts and quick checks | `moonshine-install --disk /dev/sda …` |

> ⚠️ The installer **erases the chosen disks completely**. Before installing from a config, run `moonshine-install check moonshine.toml --plan`: it shows what will be done and changes nothing.

---

## 🧰 What you need

1. A computer or virtual machine booted from the evillin install media. Any running system with `dnf` and the `moonshine-install` package works too.
2. Root rights.
3. Access to the evillin repository: over the network or in a local directory.
4. A disk whose data you do not need.

---

## 🚀 First install (wizard)

1. Boot from the install media and log in as `root`.
2. Start the wizard:

```sh
moonshine-install
```

3. Go through the eight steps. The steps are on the left, the fields of the current step on the right:

| Step | What to choose | If unsure |
|---|---|---|
| 1. Encryption | whether to encrypt the disk and how to open it | keep "Yes" and set a passphrase: it is asked at every boot |
| 2. Disk | the scheme, the system disk, the boot disk | "One disk", the right disk from the list, "Same disk" |
| 3. Repositories | where packages come from and which to install | keep everything as is |
| 4. System and user | hostname, user, password, time zone | enter the user `root` if `sudo` is not in the package list |
| 5. Network in initramfs | network before the disk is unlocked | "none" unless you need unlocking over SSH |
| 6. Network in system | network after boot | "DHCP" |
| 7. Packages | the final package list with dependencies | just look |
| 8. Summary | overview and confirmation | read the ▲ warnings |

4. On the "Summary" step type `YES`: the install starts and every action is shown on screen.
5. When "Done" appears, remove the media and reboot.

Main keys:

| Keys | Action |
|---|---|
| `↑` `↓` | field or list item |
| `Enter` / `→` | edit a field, open a list, choose |
| `←` / `Esc` | back, cancel |
| `Space` | toggle "Yes/No", open a list |
| `Tab` / `Shift+Tab` | next / previous step |
| `e` / `F2` | edit the value as text |
| `Ctrl+C` | quit without changes |

---

## 📄 Install from a config

1. Take the example [`dist/examples/moonshine.toml`](dist/examples/moonshine.toml) (after installing the package it is in `/usr/share/doc/moonshine-install/`).
2. Set the disk, the hostname and the repository.
3. Get a password hash: the command asks for the password and prints a `$6$…` line.

```sh
openssl passwd -6
```

4. Put the hash into `hash`:

```toml
disk     = "/dev/sda"
hostname = "moonshine"

[[repository]]
id      = "evillin"
baseurl = "https://repo.evillin.online/$basearch/"
gpgkey  = "file:///etc/pki/rpm-gpg/RPM-GPG-KEY-evillin"

[[users]]
name = "root"
hash = "$6$…"
```

5. Check the config. Disks are not touched:

```sh
moonshine-install check moonshine.toml --plan
```

6. Install:

```sh
moonshine-install --config moonshine.toml
```

Before erasing, the installer asks you to type `YES`. The `--yes` flag skips the question: use it for unattended installs.

The config does not have to be a local file:

```sh
moonshine-install --config https://evillin.online/moonshine.toml
moonshine-install --config 'git+https://git.evillin.online/infra.git#main:machines/moonshine.toml'
```

---

## 📖 Glossary

| Word | Meaning |
|---|---|
| **Repository** | a server or directory with `.rpm` packages; `dnf` builds the system from them |
| **Password hash** | an encrypted form of a password (`$6$…`); the password itself is not stored in the config |
| **LUKS** | disk encryption; without the passphrase or key the data cannot be read |
| **LUKS header** | the service part of an encrypted disk; it can be kept separately, for example on a USB stick |
| **initramfs** | a small system that boots first: it opens the disks and hands over to the main system |
| **dracut** | the program that builds the initramfs |
| **/boot** | the partition with the kernel and bootloader; never encrypted; here it is also the UEFI ESP |
| **BIOS / UEFI** | ways a computer boots; on x86_64 both are installed at once (hybrid) |
| **U-Boot** | the bootloader of ARM boards |
| **LVM** | volumes that can be split and grown on top of one or more disks |
| **RAID** | several disks acting as one: a mirror (RAID1) or striped |
| **wheel / sudo** | the administrators group and the program that gives them root rights |

---

## 📦 Packages

The **base set** is always installed. In the wizard it is the "Packages" field on the "Repositories" step, in a config it is `base`:

```text
bash coreutils dnf dracut evillin-fonts-console evillin-release kbd kernel lvm2 openssh-server rpm shadow-utils systemd util-linux
```

**By condition** the installer adds what the system cannot boot without:

| Condition | Packages |
|---|---|
| GRUB (BIOS/UEFI) | `grub-tools`, `grub-*-modules` per platform |
| U-Boot | `uboot-tools` |
| `/boot` partition | `dosfstools` |
| Encryption | `cryptsetup`, `lvm2` |
| LVM / thin volumes | `lvm2` / `device-mapper-persistent-data` |
| RAID | `mdadm` |
| ext4 / xfs / btrfs | `e2fsprogs` / `xfsprogs` / `btrfs-progs` |
| Network in initramfs | `dracut-network` |
| SSH keys for unlocking | `dracut-sshd`, `openssh-server` |
| Network via NetworkManager | `NetworkManager` |
| Network via systemd-networkd | `systemd-networkd`, `systemd-resolved` when DNS servers are set manually |

A package already in the list is not added again.

| What | Flag / key |
|---|---|
| Replace the base set | `--base NAME` / `base = […]` |
| Add packages | `--package NAME` / `package = […]`: a name, `@group` or a pattern |
| Only your packages, no base set and no conditional ones | `--bare` / `bare = true` |
| GRUB name (`NAME-tools`, `NAME-install`, `/boot/NAME`) | `--grub NAME` / `grub`, `grub` by default |

---

## 👤 Users and rights

```toml
[[users]]
name = "root"
hash = "$6$…"

[[users]]
name = "alice"
hash = "$6$…"
```

1. `root` in the list gets a password. If `root` is not in the list, it is **locked**.
2. Other users get a home directory and the `wheel` group.
3. A `wheel` user gets root rights only through `sudo`, and `sudo` is not in the base set. Add `sudo` to the packages or list the `root` user. If you do neither, nobody gets root rights: `check` and the "Summary" step warn about it.
4. Without a hash the password is asked on the console.

---

## 🔐 Encryption

Any device can be encrypted: a partition, a RAID array, an LVM physical or logical volume.

| Method | Key | At boot |
|---|---|---|
| Passphrase | `crypt=pass` | the passphrase is typed on the console or over SSH |
| Key file | `crypt=key` | the key is in the initramfs on the boot disk, nothing to type |
| Detached header | `header=detached` | the header is in `/etc/luks` and in the initramfs |

- For a simple scheme flags are enough: `--no-encryption` (do not encrypt), `--keyfile` (key file), `--detached-header` (detached header).
- One passphrase for all `crypt=pass` devices.
- `/boot` and `biosboot` are never encrypted.
- At boot the initramfs opens only the root and `/usr`; the system opens the rest from `/etc/crypttab`.
- A key file for the root requires a detached header.

**Unlocking over SSH.** `--ssh-key "ssh-ed25519 …"` installs `dracut-sshd`: after power-on you connect over SSH and type the passphrase. It needs network in the initramfs (`[initrd]`).

**Booting from a USB stick.** Set `--boot-disk` or "Disk for the kernel and bootloader" in the wizard. The stick gets the kernel, the bootloader and the initramfs with keys and headers. The system disk stays without a partition table: the system does not boot without the stick.

> ⚠️ If you lose the stick with the key and header, the data cannot be recovered. Keep a copy of the headers elsewhere.

---

## 💾 Disks and layout

A disk is given by path: `/dev/sda`, `/dev/nvme0n1`. A path from `/dev/disk/by-id/…` is safer: it does not change between boots.

**Ready schemes:**

| Scheme | Result |
|---|---|
| One disk | `biosboot` 4 MiB (for BIOS) + `/boot` 512 MiB + root |
| Separate boot disk | `/boot` on the second disk, the whole first disk for the system |
| LVM, RAID + LVM | in the wizard, from several disks |
| U-Boot | `/boot` as the first partition with a 16 MiB gap for the board bootloader |

`/boot` is a FAT32 partition and also the ESP. Size: `--boot-size` (from 256, 512 MiB by default).

### Custom layout

In the wizard: step "Disk" → "Disk scheme" → "Custom layout" → "Open the editor". In flags and configs there are three kinds of lines:

```text
--partition disk=DISK,size=SIZE,point=PURPOSE[,fs=ext4|xfs|btrfs][,mkfs=…][,options=…][,crypt=pass|key[,header=detached]]
--raid      name=NAME,level=0|1|5|6|10,point=PURPOSE[,fs=…][,mkfs=…][,options=…][,crypt=…]
--volume    vg=GROUP,name=NAME,size=SIZE,point=PURPOSE[,pool=POOL][,fs=…][,mkfs=…][,options=…][,crypt=…]
```

| `size` | Meaning |
|---|---|
| `512MiB`, `40GiB` | exact size |
| `rest` | all remaining space |
| `whole` | the whole disk, no partition table |

| `point` | Meaning |
|---|---|
| `/`, `/home`, `/var`, … | a file system mounted there |
| `boot` | `/boot`, FAT32, always a plain partition |
| `biosboot` | 4 MiB for GRUB when booting via BIOS |
| `swap` | swap |
| `lvm:GROUP` | a physical volume of an LVM group |
| `raid:NAME` | an array member |
| `pool` | an LVM thin pool |
| `reserve` | an empty partition the size of the boot ones, so RAID disks stay identical |

Example: LVM on one disk, `/home` on xfs.

```text
--partition disk=/dev/sda,size=4MiB,point=biosboot
--partition disk=/dev/sda,size=512MiB,point=boot
--partition disk=/dev/sda,size=rest,point=lvm:vg
--volume vg=vg,name=root,size=40GiB,point=/
--volume vg=vg,name=home,size=rest,point=/home,fs=xfs
```

Example: a two-disk mirror under LVM, booting from a USB stick.

```toml
firmware = "uefi"

[[partition]]
disk  = "/dev/disk/by-id/usb-SanDisk_Cruzer-0:0"
size  = "512MiB"
point = "boot"

[[partition]]
disk  = "/dev/sda"
size  = "whole"
point = "raid:md0"

[[partition]]
disk  = "/dev/sdb"
size  = "whole"
point = "raid:md0"

[[raid]]
name  = "md0"
level = 1
point = "lvm:vg"

[[volume]]
vg    = "vg"
name  = "root"
size  = "50GiB"
point = "/"

[[volume]]
vg    = "vg"
name  = "home"
size  = "rest"
point = "/home"
```

**Thin volumes.** Space for them is allocated as data is written. The pool is a volume with `point=pool`; a thin volume names it in `pool=`. A thin volume needs a number as its size, `rest` is not allowed. The pool is not encrypted and `swap` is never thin. If thin volumes add up to more than the pool, `check` warns.

**File system options.** `mkfs=` passes arguments to `mkfs.*`, `options=` sets the `fstab` mount options (`defaults` by default). A comma inside a value is written as `;`. `-U`, `-F`, `-f`, `--uuid`, `--force` are not allowed.

```text
--partition disk=/dev/sdb,size=whole,point=/data,fs=xfs,mkfs=-m crc=1;finobt=1,options=noatime;discard
```

**Checked before erasing:** one `/` and one `boot`; `biosboot` only with BIOS; RAID members on different disks; at most one `rest`; no repeated mount points; during the install, sizes against the real disks.

### Layout editor

| Keys | Action |
|---|---|
| `↑` `↓` `Home` `End` `PgUp` `PgDn` | move |
| `→` / `←` | expand / collapse a branch |
| `NUM+` / `NUM-` | expand / collapse everything |
| `n` | add in place |
| `r` / `v` | RAID array / LVM volume |
| `e` (`F2`) | edit as a line |
| `f` | change the file system |
| `o` | options: default → SSD → large files |
| `c` / `h` | encryption / detached header |
| `d` (`Delete`) | delete |
| `Tab` | insert `point=` |
| `u` | undo, up to 32 steps |
| `Esc` (`q`) | close |

Layout errors are highlighted in red right in the tree.

### Tools of the installer itself

If the media lacks a needed tool, the installer installs it with `dnf` before touching any disk: `mkfs.fat` (`dosfstools`), `mkfs.ext4` (`e2fsprogs`), `mkfs.xfs` (`xfsprogs`), `mkfs.btrfs` (`btrfs-progs`), `cryptsetup`, `pvcreate` (`lvm2`), `mdadm`, `thin_check` (`device-mapper-persistent-data`).

---

## 🌐 Network

The network is set up separately in two places:

| Where | Why | Flags / config section |
|---|---|---|
| initramfs | unlocking over SSH, config over the network | `--initrd-*` / `[initrd]` |
| system | normal work | `--system-*` / `[system]` |

| Mode | What to set |
|---|---|
| `none` | nothing, no network |
| `dhcp` | nothing; `interface` is optional |
| `manual` | `interface`, `address` (`10.0.0.5/24`), `gateway`, `dns` (several allowed) |

```toml
[system]
network   = "manual"
interface = "enp3s0"
address   = "10.0.0.5/24"
gateway   = "10.0.0.1"
dns       = ["9.9.9.9"]
backend   = "networkd"
```

`backend` is the network service: `networkd` (default) or `networkmanager`. The other service is disabled (`systemctl mask`), even if it is not installed.

---

## 🧩 Kernel and services

| What | Flag | Config | Wizard |
|---|---|---|---|
| Kernel parameters | `--kernel-arg quiet` | `kernel_arg = ["quiet"]` | "Kernel parameters" |
| Services | `--service sshd` | `service = ["sshd"]` | "Services to enable" |

- Kernel options are saved in `/etc/default/grub` and `/etc/kernel/cmdline`: new kernels get them automatically.
- Services are enabled with `systemctl enable`. If a service's package is not in the base set, add it to the packages.

---

## 📚 Repositories

| From | Flag / key | Kept in the system |
|---|---|---|
| A repository of the media (`/etc/yum.repos.d`) | `--repo ID` | with `--repo-file` or "Copy repositories to the system" in the wizard |
| A local directory with `.rpm` files and metadata | `--repo-dir PATH` | no |
| Described in the config | `[[repository]]` / `--repository …` | yes |

```toml
[[repository]]
id       = "evillin"                                      # required, unique
baseurl  = "https://repo.evillin.online/$basearch/"       # or metalink, or mirrorlist
gpgkey   = "file:///etc/pki/rpm-gpg/RPM-GPG-KEY-evillin"  # needed while gpgcheck is on
priority = 10                                             # 1…99, lower wins
```

More keys: `name`, `exclude`, `includepkgs`, `gpgcheck = false`.

- Priority of a media repository for the install only: `--repo-priority ID=N`.
- `dnf` options: `--option key=value`, for example `tsflags=nodocs`. There are none by default.

---

## 🗂️ Config in detail

A config is a TOML file. Its keys mirror the flags:

1. `boot_disk = "…"` is the same as `--boot-disk …`.
2. An array repeats the flag: `kernel_arg = ["quiet", "splash"]`.
3. `true` sets a flag (`keyfile = true`), `false` does nothing.
4. The `[initrd]`, `[system]`, `[uboot]` sections add a prefix: `[system] network = "dhcp"` is `--system-network dhcp`.
5. The `[[users]]`, `[[partition]]`, `[[raid]]`, `[[volume]]`, `[[repository]]` blocks can be repeated.

Flags given on the command line after `--config` win over the config.

```toml
disk      = "/dev/disk/by-id/nvme-Samsung_SSD_980_PRO"
boot_disk = "/dev/disk/by-id/usb-SanDisk_Cruzer-0:0"
hostname  = "moonshine"
timezone  = "Europe/Moscow"
keyfile   = true
ssh_key   = ["ssh-ed25519 AAAA… alice@evillin.online"]
package   = ["sudo"]

[[repository]]
id      = "evillin"
baseurl = "https://repo.evillin.online/$basearch/"
gpgkey  = "file:///etc/pki/rpm-gpg/RPM-GPG-KEY-evillin"

[[users]]
name = "alice"
hash = "$6$…"

[initrd]
network = "dhcp"

[system]
network = "dhcp"
```

A config from git: `git+URL#[branch|tag|commit:]path.toml`, a path inside the repository, no `..`.

---

## ✅ Checking: check

`check` validates the whole config and shows what will happen. **Disks are neither changed nor read.** Exit code `0` means valid, `1` means an error.

```sh
moonshine-install check moonshine.toml          # overview
moonshine-install check moonshine.toml --plan   # overview and every command in order
```

```text
The configuration is valid.

Platform:       BIOS+UEFI · x86_64
Hostname:       moonshine
Time zone:      Europe/Moscow
Users:          root, alice
Kernel options: none
Services:       none

Layout (disks and their sizes are not read during a check):
  /dev/sda1                4MiB  biosboot
  /dev/sda2              512MiB  boot
  /dev/sda3                rest  /  [LUKS: key file, detached header]

Package sources:
  config repository evillin
  base set: bash coreutils dnf dracut evillin-fonts-console evillin-release kbd kernel lvm2 openssh-server rpm shadow-utils systemd util-linux
  extra packages: none
  GRUB: grub
Network in initramfs: none
Network in system:    dhcp
```

When something is worth knowing in advance, a "Notes" block appears at the end. Notes do not stop the install.

---

## 🔑 Passwords and keys

| What | How to pass it |
|---|---|
| User password | an `openssl passwd -6` hash: `hash` in a config or `--user alice:'$6$…'` |
| Disk passphrase | a file with mode 600: `--passphrase-file` / `passphrase_file`; without a file it is asked on the console |
| Disk key | created automatically with `crypt=key` |
| SSH key for unlocking | `--ssh-key` / `ssh_key` |
| Access to a config over `https` | `~/.netrc` |
| Access to git | an SSH key or a git credential manager |

- A passphrase cannot be a hash: the encryption key is derived from the phrase itself.
- A passphrase file readable by group or others, empty or with several lines is refused.
- Never put a password into a URL.

> ⚠️ A config served over the network contains password hashes: serve it over `https`.

---

## 💿 Installing from media and over the network

The installer can live inside an initramfs: it starts right after boot, from a USB stick, over the network (PXE/iPXE) or from a GRUB menu entry.

1. Build and install the files:

```sh
cargo build --release --locked
make -C dist install
```

2. Build an initramfs image with the installer:

```sh
/usr/libexec/moonshine-install/build-initrd moonshine-installer.img
```

3. Boot a kernel with this image and these options:

| Kernel option | What it does |
|---|---|
| `moonshine.config=WHERE` | install from a config: a file, `https://…` or `git+…` |
| `moonshine.wizard=1` | the wizard on the console |
| `moonshine.yes=1` | no `YES` question |
| `moonshine.check=1` | only check the config |
| `moonshine.after=reboot\|poweroff\|shell\|none` | what to do after success, `reboot` by default |
| `moonshine.lang=en\|ru` | language |

- A config over the network needs a network: without `ip=` the module adds `rd.neednet=1 ip=dhcp` itself.
- A passphrase file goes into the image like this: `build-initrd inst.img "$(uname -r)" --include /root/luks.txt /root/luks.txt`.
- On failure the dracut emergency shell stays open.

**A GRUB menu entry on a running machine.** Handy for reinstalling a remote server: after a reboot you pick the entry and the machine installs from the config. `moonshine.yes=1` is needed when nobody is at the console: the disk is erased without asking.

```sh
/usr/libexec/moonshine-install/grub-entry /vmlinuz /moonshine-installer.img \
    https://evillin.online/moonshine.toml moonshine.yes=1 >> /boot/grub/custom.cfg
```

**iPXE:**

```text
#!ipxe
kernel http://evillin.online/vmlinuz moonshine.config=http://evillin.online/moonshine.toml moonshine.yes=1 ip=dhcp
initrd http://evillin.online/moonshine-installer.img
boot
```

**Live system.** The service starts the installer from the same kernel options:

```sh
systemctl enable moonshine-install.service
```

`make -C dist install` understands `DESTDIR` and `PREFIX`. Script checks: `make -C dist check`.

---

## 🌍 Language

Russian is picked automatically when the system language is Russian (`LANG=ru_RU.UTF-8`), English otherwise. Explicitly: `--lang ru`, `lang = "ru"` in a config, the `MOONSHINE_LANG=ru` variable or the `moonshine.lang=ru` kernel option.

---

## 📋 All flags

```text
moonshine-install                              wizard
moonshine-install FLAGS                        install
moonshine-install check TARGET [--plan] [FLAGS]  check

Disks
  --disk PATH                   system disk
  --boot-disk PATH              boot disk (same disk by default)
  --boot-size MIB               /boot size, from 256 (512 by default)
  --partition …  --raid …  --volume …   custom layout
  --firmware hybrid|uefi|bios|uboot
  --arch x86_64|aarch64
  --uboot-image FILE            U-Boot image for the board
  --uboot-offset SECTORS        image offset, below 32768

Encryption
  --no-encryption               do not encrypt the root
  --keyfile                     open the root with a key file
  --detached-header             detached root header
  --ssh-key "KEY"               unlocking over SSH (repeatable)
  --passphrase-file PATH        passphrase file (mode 600)

Packages
  --repo ID                     media repository (repeatable)
  --repo-priority ID=N          its priority 1…99 for the install
  --repo-file FILE              copy a media .repo into the system (repeatable)
  --repo-dir DIR                local package directory (repeatable)
  --repository id=…,baseurl=…   repository from a description (repeatable)
  --base NAME                   base set instead of the built-in one (repeatable)
  --package NAME                extra package (repeatable)
  --bare                        no base set and no conditional packages
  --grub NAME                   GRUB name (grub by default)
  --option KEY=VALUE            dnf option (repeatable)

System
  --hostname NAME
  --timezone ZONE               UTC by default
  --user NAME[:HASH]            user (repeatable)
  --kernel-arg OPTION           kernel option (repeatable)
  --service UNIT                enable a service (repeatable)

Network
  --initrd-network none|dhcp|manual    network in initramfs
  --system-network none|dhcp|manual    network in system
  --initrd-/--system- interface, address, gateway, dns
  --system-backend networkd|networkmanager

Other
  --config WHERE                file, https:// or git+URL#[branch:]path
  --lang en|ru                  language
  --yes                         no YES confirmation
```

---

## ❓ FAQ

**How do I see what will be done without erasing anything?** `moonshine-install check moonshine.toml --plan`.

**After the install I cannot become root.** A `wheel` user gets root rights only through `sudo`. Log in as `root` (if you gave it a password) and install `sudo`, or reinstall with `sudo` added to the packages.

**A password is asked at boot.** It is the encryption passphrase set on the "Encryption" step. To avoid typing it, choose a key file: the key will be on the boot disk.

**The "Packages" step shows an error.** Usually a listed package is missing from the repository or a dependency is missing. The error text is the `dnf` output. Remove the package from the "Packages" field or add the needed repository.

**Which disk to choose?** The wizard shows each disk's type (SSD/HDD), size and model. With flags a path from `ls -l /dev/disk/by-id/` is safer.

---

## 🧪 For developers

```sh
cargo build --release
cargo fmt && cargo clippy --all-targets && cargo test
make -C dist check
```

- Rust edition 2024. Tests live in `tests` modules and do not need root.
- The host needs `dnf`, `sfdisk`, `wipefs`, `mount`, `dracut`; `curl` and `git` for configs over the network.
- The `sources_use_only_glyphs_of_the_console_font` test keeps the interface to the characters of the evillin console font.

Design: a config from the wizard, flags or TOML becomes a plan, a list of steps. The plan runs in order: releasing disks → partitioning → RAID → LVM → LUKS and formatting → mounting → `dnf` → configuration → `dracut` → bootloader → unmounting. Passwords reach commands through stdin; user data never reaches a shell.

| Module | Purpose |
|---|---|
| `main`, `cli`, `check` | entry, modes, report |
| `plan`, `storage`, `layout` | steps, storage, layout model |
| `config`, `options`, `manifest`, `secrets` | settings, flags, TOML/URL/git |
| `repository`, `repos`, `sources`, `resolve` | repositories, package resolution |
| `hardware`, `disks`, `network` | hardware, disks, network |
| `draft`, `app`, `ui`, `tui`, `editor`, `input` | wizard and editor |
| `exec`, `error`, `i18n` | execution, errors, language |

---

## 🚧 Limitations

- Architectures: `x86_64`; `aarch64` is in testing.
- File systems: ext4, xfs, btrfs (no subvolumes).
- Configs over a URL or from git are not checked by signature.
- An LVM group that also spans another disk stays partially active there.
- Not yet tried on real hardware: several encrypted devices in the initramfs, LUKS on RAID and LVM, thin volumes, btrfs, U-Boot, the dracut module and the systemd service on a finished image.

---

## 📜 License

This project is licensed under the Apache License 2.0. You may freely use, distribute and modify it under the terms of the license.
For details see the [`LICENSE`](LICENSE) file or visit the [Apache website](https://www.apache.org/licenses/LICENSE-2.0).

---

## 👤 Contact

Author: NSerr0R  
Email: [nserr0r@gmail.com](mailto:nserr0r@gmail.com)
