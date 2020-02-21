# tewa-708e破解流程
1. 路由器插入u盘，fat32文件系统。
1. 常规用户登陆http://192.168.1.1:8080
1. 切换到 管理-设备管理
1. 浏览器进入调试模式，在console处，从top切换到MD_Device.html
1. 在console内执行
```
`http://192.168.1.1:8080/usbbackup.cmd?action=backupeble&sessionKey=${sessionKey}`
```
1. 注意反引号属于代码的一部分，得到一个url，打开之后点击备份
1. 在u盘内得到ctce8_TEWA-708E.cfg文件
1. 使用router data view或者附件的xor解密
1. 在解密后文件内找到TeleComAccount，下面的Password内即为超密
1. 如果需要打开telnet，超级用户登陆后，打开 http://192.168.1.1:8080/enableTelnet.html 即可启用telnet
