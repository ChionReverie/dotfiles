# Create Users

Create your user with any groups you may need. 
Note that the `wheel` group grants access to sudo capabilities.

Example for a user named chion:

```sh
useradd chion --create-home --groups wheel,ftp
passwd chion
```
