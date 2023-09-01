# ZOS-Index-demo

[![Logo](./assets/img/logo1_small.png "Logo")](file:///C:/Users/AdminJason/Desktop/ZOW/Zeal-8-bit/ZOS-Index-demo/assets/img/logo1.png)

Here is the index of the prebuild mirrors list of Zeal-8-bit-Computer.

## Write an index by yourself

name: The name will show in the select of the emulator.

urls: Where is your image? Should be an url to the image.

version: Version of your image. You can find system info in zos at 0x2f3.

hash: The hash value of your image. Only support SHA256 now. You can run following commands on Windows to check your hash value:

```powershell
certutil -hashfile /path/to/your/image SHA256
```

or use following commands on linux instead:

```bash
sha256sum /path/to/your/image
```

And push it to the repo, finish!

## TODO

Add CORS test script
