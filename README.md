# Rivals Constant Flattener

RCF is a simple, easy-to-use export tool for optimizing constant values in Rivals of Aether characters. RCF will reference values from a source ``user_event`` file of your choice and bake them directly into your project's scripts, allowing you to write far more readable code without incurring performance hits from GML's automatic instance checks.

RCF also comes bundled with tools for easy version changes and automatic file deletion, making it easy to convert your character's development build into an upload-ready state.

> RCF is designed to minimize the risk of file loss. However, it is always good practice to back up your data before using a tool of this nature. Do not overwrite your development build with an RCF build!

## Configuring a project for RCF
In order to use RCF, you'll need to choose a source ``user_event`` script to host your constants. This script should be called from near the top of your character's ``init.gml`` file, which will allow you to fully playtest the character in a pre-export state.

The ``user_event`` script itself can be written in mostly the same way that you would write an init file. However, there's a few things you should keep in mind when doing so:
- The RCF ``user_event`` file should only contain constant values. Attempting to alter one of these constants later will cause the RCF export to fail.
- It's best practice to write your constant names in ``ALL_CAPS``, which makes them immediately identifiable as constants and maintains consistency with Rivals' default set of constants.
- RCF functions as a series of macros, so the contents of these variables will be directly copied into the code with no further processing. As such, it's strongly recommended to only use RCF to define primitives or strings.
	- \[Implementation note: RCF should explicitly prevent use of common object creation functions, along with hit_fx_create().\]

## Exporting a project through RCF
To use RCF, you'll need to input your project's source directory. From there, several options will open up:
- Output directory: Allows you to change the output directory. Note that RCF will refuse to export to a directory that already exists.
- Strip comments: If enabled, RCF will remove all comments from your code. Rarely recommended.
- Minimize whitespace: If enabled, RCF will attempt to minimize the amount of whitespace in your code. Almost never recommended, as it will make the exported code nearly impossible to navigate.
- Character properties: Allows you to inspect and edit information from your character's config.ini file, such as gameplay hints and Steam tags.
> Note that when using the "Export" option, RCF will decrease the "minor version" value by 1. This is because Rivals automatically increases this value by 1 when uploading to the Steam Workshop, thus matching the value listed in RCF.
- Generate include file: Generates a default ``rcf_include.txt`` file in your project's source directory. This will be done automatically on export if necessary, so you only need to do this if you're planning to edit the include file.
- Update config.ini: Updates your character's source config.ini file using the character properties listed in RCF. Note that the exported config file will always use the edited properties, regardless of whether this button is used.
- Export: Processes and exports your character to the chosen output directory.

## Advanced functionality

### Running development-only init code
In addition to its typical macro functionality, RCF is able to accomodate code from its source ``user_event`` file. Since this code will be lost on export, it's a great place to host code that you'd only like to include in development builds, such as enabling a debug flag.

In order to do this, include the line ``//#RCFBEGINIGNORE`` before your desired block of code. If you'd like to place additional macros below the ignored code block, you can use ``//#RCFENDIGNORE`` to do so.

> ``IGNORE`` tags will also work perfectly fine in other scripts, but this isn't recommended, as having ignorable code inline with your regular code is likely to create bloat and headaches. Binding development behavior to a debug flag and/or hiding it in a ``user_event`` file is typically much better practice.

### Including additional files
RCF will usually strip all unnecessary files and folders from the root directory while exporting. However, there are certain extra files that can be useful to include in an export, such as a changelog or a color-mapped portrait. In these cases, ``rcf_include.txt`` can be edited to include these extra files.

Entries in the include file consist of their file or folder name, separated by whitespace. Files containing spaces should be enclosed in quotation marks. Note that all of the contents of any included folder will be exported, assuming that they have a supported file type.

Note that * can be used as a wildcard character to denote that files of a given type should be retained. For example, the default include contains the entries ``"scripts/*.gml"`` and ``"scripts/attacks/*.gml"``.

> RCF strictly obeys the contents of the include file. As such, deleting entries from the default include file will typically cause problems and is highly discouraged.

### Regional deformatting
While the "strip comments" and "minimize whitespace" checkboxes are too over-the-top for most use cases, there's some instances where localized segments of formatting will break down after being flattened, such as tab-aligned tables containing a lot of constants. If you'd like to specifically strip comments and whitespace from these areas, you can wrap them in ``//##RCFBEGINDEFORMAT`` and ``//#RCFENDDEFORMAT``.

Of course, this will make the selected region hard to navigate if something needs to be checked on the exported build, so this functionality should preferably only be used to clean up data initializations. It'll also cause an error if used in the RCF source ``user_event``, which doesn't generate any output files to be deformatted.