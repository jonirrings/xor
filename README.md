# 思路
1. 寻找备份页面URL的参数，从而进入备份页面
1. 备份配置文件
1. 解密配置文件，得到账号密码
# tewa-708e破解流程
1. 路由器插入u盘，fat32文件系统。
1. 常规用户登陆http://192.168.1.1:8080
1. 切换到 管理-设备管理
1. 浏览器进入调试模式，在console处，从top切换到MD_Device_user.html
    1. 在console内执行
        ```
        `http://192.168.1.1:8080/usbbackup.cmd?action=backupeble&sessionKey=${sessionKey}`
        ```
    1. 注意反引号属于代码的一部分，得到一个url，打开之后点击备份(如果不可点击，审查元素然后删除disabled)
    1. **如果上述方式执行失败**，根据[KoolShare](https://koolshare.cn/forum.php?mod=redirect&goto=findpost&ptid=166510&pid=2227183)坛友的做法也可以尝试执行下列代码
        ```js
        usbsubarea = {
          selectedIndex: 0,
          value: "never_mind...",
          options:[
            {value: "usb1_1"}
          ]
        };
        
        btnApply();
        ```
    1. 如果都失败，请参考[相近型号破解的思路](#其他几个相近型号的破解)和[#2](https://github.com/jonirrings/xor/issues/2)
1. 在u盘内得到ctce8_TEWA-708E.cfg文件
1. 使用[routerpassview](http://www.nirsoft.net/utils/router_password_recovery.html)或者[附件的xor](https://github.com/jonirrings/xor/releases)解密， 使用方法参考[#4](https://github.com/jonirrings/xor/issues/4)
1. 在解密后文件内找到TeleComAccount，下面的Password内即为超密
1. 如果需要打开telnet，超级用户登陆后，打开 http://192.168.1.1:8080/enableTelnet.html 即可启用telnet

# 关于本项目
备份的cfg文件只是被简单地 `byte ^ 0xff` 处理过，所以根据`xor`的特性，再次计算就能得到原始数据。

# 编译
有rust环境的话，直接`cargo run -- cfg文件`就行。
没有环境的话，直接下载[release页面](https://github.com/jonirrings/xor/releases)已经编译好的执行文件，然后`cmd`内执行`xor cfg文件`就行。

# 其他几个相近型号的破解
关于tewa-1000e, tewa-800e之类的型号，将console内执行那一步的url改一下，可以参考[#2](https://github.com/jonirrings/xor/issues/2)：
1. 在`MD_Device_user.html`里面找到的`backupeble`这个关键字
1. 找到跟`session`相关的参数，手动拼凑有效的url
