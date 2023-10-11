# IIPDP
an Incredibly Inefficient Portal Demo Parser written in Rust

# Installation
1. Download the .exe file from the Releases tab
2. That's it!

# Usage
## Windows
Open the demo file with `iipdp.exe`. 

You can open it from the command line:
`iipdp.exe <demo name> [options]`

- Options:
`-dump`: Outputs all available information from the demo file to a .txt file in the working directory.

Or just drag a demo file onto it to open it.

## Linux
Terminal: `./iipdp <demo name> [options]`

Options are the same as Windows

# Compatible Source Engine Versions
Portal:
* 5135 (Source Unpack) - yes

Everything else:
* No
# Changelog
- v0.1:
    * Added basic packet reading, the demo dumping feature and demo timing.
    * First release
- v0.2:
    * Added NET/SVC message parsing
    * Completely restructured the original packet reading code
- v0.2.1:
    * Made it go vrooom (very fast)
