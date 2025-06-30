All binaries print their argvs.
They also print what was observed on Linux when invoking them from bash with no slashes on the command line with them added to the PATH.
TODO: Double check the interpretation of what's going on in python3 and bash. I haven't found that documented, but I haven't looked - it's just my best guess at unifying the behavior I'm seeing. Try executing with './' in the PATH and "${PWD}" in the PATH to replicate.

If you invoke any of these programs through a hardlink or a softlink, the name they print is derived from the name of the link. (With hardlinks, I can't see any other way. With softlinks, I could see different behavior, but I'm glad I don't - the `argv[0]` passed to `execve` is meant to be significant as text, and all the languages other than `swift` let you obtain that text from the value they present to the program. Maybe some path manipulation is required, but still.).
