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
`-dump`: Outputs all available information from the demo file to a .txt file in the working directory. Has sub-options `-fc` (dump flattened classes) and `-v` (verifier dump)
Usage example: `./iipdp <demo name> -dump [-v/-fc]`

Or just drag a demo file onto it to open it.

## Linux
Terminal: `./iipdp <demo name> [options]`

Options are the same as Windows

# Compatible Source Engine Versions
Portal:
* Steampipe - yes
* 5135 (Source Unpack) - yes
* 3420 - yes

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
- v0.2.2:
    * Added Steampipe and 3420 support
    * Made GameEventList implementation better
- v0.2.3:
    * Added UserMessage parsing and dumping
- v0.2.4:
    * Added SvcBspDecal message parsing
    * Added PortalFXSurface user message parsing
    * Added support for parsing directories of demos
    * Made time formatting nicer
- v0.2.5:
    * Added Stringtables packet parsing and dumping
- v0.2.6:
    * Added time adjustment, now this parser can actually time runs properly
- v0.2.7:
    * Added Datatables packet parsing and dumping
    * Prettier flag dumping
    * Made it go vroooooooom (very fast) (again)
- v0.3.0:
	* Added verifier dump feature `-v`