#### 功能介绍

如描述。这个命令行工具通过调用腾讯云机器翻译API实现简单翻译。
TODO：添加词典模式和文本翻译模式

#### 项目配置

##### 配置 api key

注册腾讯云API，生成 `secret_id` 和 `secret_key`,创建环境变量 `TCC_SECRET_ID` 和 `TCC_SECRET_KEY`。

##### 构建cli

从源码构建 `绝对不是我懒得交叉编译release (*/ω＼*)`  
`git clone XXXXXX`
`cargo build -r`
在 `./target/release/`里面找到可执行文件 copy&exec 

#### 使用方法

    rust-trans-cli.exe -p <plain> -f <from_lang> -t <to_lang>  # 使用指定的语言策略翻译 plain
    rust-trans-cli.exe -c <config_name> -s -f <from_lang> -t <to_lang>  # 将翻译策略保存至配置
    rust-trans-cli.exe -c <config_name> -d  # 删除翻译策略配置
    rust-trans-cli.exe -c <config_name> -p <plain>  # 使用翻译策略进行翻译
