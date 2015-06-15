# Dotefiles Client
Dotefiles Client is a program designed to synchronize configurations and packages which is aimed primarily at Arch Linux users.
It is written in Rust, and implements a custom file format that is used for synchronization tasks such as installing certain fonts and packages or creating a symlink to an automatically updated file, or even cloning/pulling from a git-repository.

## Why does this software exist?
Simply put: Convenience. Not a long time ago I installed Arch Linux on a new Computer and while many people consider the installation process itself very frustrating, it allows for huge leeway in it's configuration and a minimal footprint whilst being more or less a one time hassle. However, when it comes to installing certain programs like the Window-manager you use on almost all your current installations on different machines or configuring your preferred shell, it does not only get repetitive and exhaustive but is also prone to error.
This is why I thought it would be nice to have exactly this kind of program, counteracting the perpetuating troubles of updating a configuration on one machine or installing an interesting package on one machine and having to explicitly do the same for all other machines you want it to be installed on.
Whilst people often create git-repositories or use similar alternatives and many of them already implement scripts fulfilling parts of or the complete role this program aspires to perform, they are in general not very applicable to another user's configuration.
This is why *reusability* is one of the key goals of this program.

## What does this software do?
All actions this program performs will be read from a configuration file, which is interpreted at runtime. The actions inhabiting this script-language are listed below, and marked ***not implemented*** in case the mentioned feature is not to be found in the actual program yet, but planned.

- Installing packages from the official repositories (***not implemented***)
- Configuring a custom package-manager, to handle packages not in the official repositories (***not implemented***)
- Removing installed packages explicitly. These are packages that should not be on any installation. A good example would be an interfering package that must be removed before a certain action can take place. (***not implemented***)
- Cloning a repository, keeping it up to date and recursively executing it's Makefile, effectively installing it automatically (***not implemented***)
- Enabling/Disabling services using systemd (***not implemented***)
- Creating Symlinks. For instance to link a .vimrc from an automatically updated repository (***not implemented***)
- Execute a custom shell-command to extend the functionality of the program. In case you feel the feature would make a useful addition to the program, consider mentioning it, since we may well simply had not thought of it (***not implemented***)

## What does this software *not* do?
It is obvious that this software does not do everything, and nobody expects it to make coffee instantaneously and from thin air (although admittedly it would be a nice feature to have), hence these things are not noted here. Albeit there are some conscious design decisions about what this program does not include even though a user might expect them to be in here somewhere. The following list consists of just these features and the reasons, why they have not been included.

- **Installing from the AUR.** There are much better clients serving just this purpose and even though they can be configured to be used as a source, they cannot be automatically installed.
- **Updating individual packages.** Again, this has nothing to do with synchronizing multiple machines. Updates should be carried out using the package manager of your choice and explicitly.
- **Updating configuration files for remote machines.** This program features no means for synchronizing your local configuration files with those of another machine, which means they have to be updated using a separate mechanism. Even though it is possible to simply put them into a repository with the program itself, this is highly discouraged mainly due inefficient updates. Consider creating an external repository for these files instead.
- **Copying or Moving files.** Only symlinks are created by this program, since they do not double the required space or remove the original file, so that all configuration can be carried out in one place and therefor synchronized much easier.
