# ZOS-Index-demo

Here is the index of the prebuild mirrors list of Zeal-8-bit-Computer.

### Feature

Add support for more urls and check for the speed

Add "authors"

Add usement to "version"

## Write

name: The name will show in the combobox of the emulator.

urls: Where is your image? Should be an url link to the image.

version: Version of your image. You can find system info in zos at 0x2f3.

hash: The hash value of your image. Only support SHA256 now. You can run following commands on Windows to check your hash value:

```
certutil -hashfile /path/to/your/image SHA256
```

## Generate

If you don't want to write your index by yourself, you can try zos-index-generator. It is a program which aims to generate index.json easier. You can get its binary in our release page or use following commands if you have installed rust:

```
cd /path/to/zos-index-generator
cargo run <zos-index-generator commands>
```

For example, if you want to add your index to index.json, run:

```
cargo run new
```

And follow the steps, zos-index-generator will push it to the branch "temp", I wll merge it when I got some times.

## TODO

There are some bugs in auto push now, I have to fix it.
