# Rivals Constant Flattener

RCF is a simple, easy-to-use export tool for optimizing constant values in Rivals of Aether projects. RCF will reference values from a source ``user_event`` file of your choice and bake them directly into your project's scripts, allowing you to write far more readable code without incurring performance hits from GML's automatic instance checks.

RCF also automates most of the irritating parts of converting a project from a development build to  a release build, including ``config.ini`` handling and safe removal of extra files.

> RCF is designed to minimize the risk of file loss. However, it is always good practice to back up your data before using a tool of this nature. Do not overwrite your development builds with RCF builds!

## What's this for, anyway?

Rivals Workshop projects don't have an especially robust way of handling user-defined constants. ``#macros`` work, but they only apply in the file where they're defined, which makes them too unwieldy for any serious use cases. Meanwhile, defining constants to typical variables works alright, but since Game Maker has to check if the variable links to a valid instance, it comes with a small performance cost, especially if referencing the variable from another object's perspective. Additionally, for more involved projects, having several hundred unchanging primitive instances sitting around around in your character object feels like kind of a waste.

RCF offers a compromise to this problem: tank the small efficiency penalty from using constant variables while you're developing your project, then automatically convert those variables to hard-coded values before uploading the project to the Workshop. While it's a small optimization, it offers the power to parameterize your project's attributes at effectively no cost, which will make maintainence and patches much easier to manage in the long run.

## Configuring a project for RCF
In order to use RCF, you'll need to choose a source ``user_event`` script to host your constants. This script should be called from near the top of your project's ``init.gml`` file, which will allow you to fully playtest the project in a pre-export state.

The ``user_event`` script itself can be written in mostly the same way that you would write an init file. However, there's a few things you should keep in mind when doing so:
- The RCF ``user_event`` file should typically only assign constant values. Attempting to alter one of these constants later will cause errors and incorrect behavior in the RCF export.
	- RCF copies the contents of these variables directly into the code with no further processing. As such, RCF will yield the best results when define primitives or strings.
	- Do not assign ``hit_fx_create()`` to an RCF variable! This function creates a hit effect index at runtime and should always be stored to a variable in ``init.gml``.
- It's best practice to write your constant names in ``ALL_CAPS``, which makes them immediately identifiable as constants and maintains consistency with Rivals' default set of constants.

Additionally, upon exporting your project, RCF will create the files ``rcf_include.txt`` and ``config_export.ini`` in your project's root. ``rcf_include.txt`` will be used in future exports to control which files are exported, while ``config_export.ini`` will be exported in lieu of your project's ``config.ini`` file, allowing you to set a distinct name and version for your release and development builds.

> RCF will automatically account for object references that link back to your constants (e.g. ``other.player_id.CONSTANT``), so feel free to reference constants from the perspectives of articles or enemy players.

## Exporting a project through RCF
Run RCF from the command line using this syntax:

``rcf.exe [source directory] [destination directory] [flags]``

The source and destination directories should be two distinct paths to folders, enclosed in quotes. Additionally, the following flags are available:
- ``-ue [#]``: Sets the user_event used for constants. Flattening will only take place if this flag is present, so it should only be omitted if you're using RCF purely as an export utility.
- ``-w``: Strips excess whitespace from your code. May provide a very slight performance boost, but will make your code much less readable. If you use this flag, please be sure to provide a GitHub link to your project's pre-export source code as a courtesy to players and tournament organizers.
- ``-c``: Strips comments on export. Serves the same purpose as stripping whitespace, and only recommended if already stripping whitespace.
- ``-s``: Silences console output. Requires use of ``-o`` or ``-safe``, since doing so prevents certain safety prompts from being displayed.
- ``-o``: Forces overwrite of destination directory, even if the target folder isn't a Rivals Workshop project. Increases likelihood of data loss, so use with care.
- ``-safe``: Prevents contents of destination directory from being overwritten.
- ``-init``: Performs a dry run that exclusively initializes the ``config_export.ini`` and ``rcf_include.txt`` files, assuming they're not already present. The ``dest`` argument can be omitted if and only if this flag is present.
- ``-inert``: Blocks new files from being written to the source folder. This flag does not otherwise affect export behavior. This flag is incompatible with ``-init``.

> Note: Only character projects (``type="0"`` in ``config.ini``) are currently supported.

## Advanced functionality

### Including additional files
RCF will usually strip all unnecessary files and folders from the root directory while exporting. However, there are certain extra files that can be useful to include in an export, such as a changelog or a color-mapped portrait. In these cases, ``rcf_include.txt`` can be edited to include these extra files.

Entries in the include file consist of file paths, separated by newlines. Note that all of the contents of any included folder will be exported, assuming that they have a supported file type.

Note that * can be used as a wildcard character to denote that files of a given type should be retained. For example, the default include contains the entries ``"scripts/*.gml"`` and ``"scripts/attacks/*.gml"``.

> RCF strictly obeys the contents of the include file. As such, deleting entries from the default include file will typically cause problems and is highly discouraged.

### Running development-only init code
In addition to its typical macro functionality, RCF is able to accomodate temporary code from its source ``user_event`` file. Since this code will be lost on export, it's a great place to host code that you'd only like to include in development builds, such as enabling a debug flag.

In order to do this, include the line ``//#RCFBEGINIGNORE`` before your desired block of code. If you'd like to place additional macros below the ignored code block, you can use ``//#RCFENDIGNORE`` to do so.

> ``IGNORE`` tags will also work perfectly fine in other scripts, but this isn't recommended, as writing ignorable code inline with your regular code is likely to create bloat and headaches. Binding development behavior to a debug flag and/or hiding it in a ``user_event`` file is typically much better practice.

### Regional deformatting
While the "strip comments" and "minimize whitespace" checkboxes are too over-the-top for most use cases, there's some instances where localized segments of formatting will break down after being flattened, such as tab-aligned tables that contain a lot of constants. If you'd like to specifically strip comments and whitespace from these areas, you can wrap them in ``//#RCFBEGINDEFORMAT`` and ``//#RCFENDDEFORMAT``.

Of course, this will make the selected region hard to navigate if something needs to be checked on the exported build, so this functionality should preferably only be used to clean up data initializations. It'll also cause an error if used in the RCF source ``user_event``, which doesn't generate any output files that could be deformatted.