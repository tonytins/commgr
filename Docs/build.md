# Building

## VSCode

This project has several build tasks:

- run install dummy
- run remove dummy
- publish debug linux x64
- publish debug windows x64
- publish release linux x64
- publish release windows x64

Publish options give additional parameters set in order to make it an ideal command line application, this includes ``PublishSingleFile`` and ``PublishTrimmed``; the former bundles the runtime with the executable while the latter removes unused assemblies. It will still be large but it won't be as large if it weren't trimmed.

The benefits of this is that the host system does not need to have the required runtime installed and this cuts down on DLL hell.