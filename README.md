# ZOS-Index-demo

Here is the index of the prebuild mirrors list of Zeal-8-bit-Computer.

### Feature

Add support for more urls and check for the speed

Add "authors"

Add usement to "version"

# Write

name: The name will show in the combobox of the emulator.

urls: Where is your image? Should be an url link to the image.

version: Version of your image. You can find system info in zos at 0x2f3.

hash: The hash value of your image. Only support SHA256 now. You can run following commands on Windows to check your hash value:

```
certutil -hashfile /path/to/your/image SHA256
```
