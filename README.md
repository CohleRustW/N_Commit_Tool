### N Commit Tool
> 本工具用于 GitHub 开源仓库开发者，开发过程中交互式选择并创建本地开发分支，并通过对应 Issue ID 的描述信息完成 commit message 拼接.
> 
> 如果你的开发工作流不是通过`github issue`进行管理，那此工具不适合你

#### 工具目录
> mac os 已编译测试版本二进制到目录 examples/ncommit
 1. 使用方式1：追加以上函数到 .bashrc or .zsh_profile
```bash
 function ncommit () {
     /Users/......./ncommit $@
 }
```

2. 方式2：放置工具在路径 `/usr/local/bin`目录下

#### 依赖步骤
1. 依赖开源工具 gh, 需要提前安装并且完成鉴权
2. 新增配置文件`ncommit.yml`到 `/etc/`目录下，根据项目需求自行配置
```bash
 ---
# git remote name on working directory
 remote_name: origin

# 通过本地分支名称提取对应的 Issue ID, 用于关联 Issue, 例如 test_issue#1111, 支持正则表达式
 dev_issue_re: .*issue#?(\d+).*

# 是否开启自动 fetch 远程分支
 enable_auto_fetch: true

# 版本比较正则，用于提取符合正则规则的最大版本号来创建 dev branch
 version_compare_re: V(\d{1,2}\.\d{1,2}\.\d{1,2})-rc

# Issue title 正则, 用于匹配 Issue title, Like: [BUG] xxxxx
 issue_title_filter_re: \[(.*)\](.*)

# 本地分支名创建规则，自动追加 Issue ID在分支名称后面
 dev_issue_name_header: dev_issue#

# 是否启动节点管理 commit message append, 默认关闭
 commit_append_nodeman_msg: false

# 节点管理 commit message append 规则
 commit_append_msg: (wf -l)

# github work flow link commit description Like：fixed close closed
 commit_link_description: closed

 ```

##### 安装步骤
1. ``brew install gh``
2. gh登录鉴权

#### 支持参数
1. 直接执行``ncommit`` -> git commit -m "bugfix: Test Issue (closed #4)(wf -l)" 参考: (issue#4)[https://github.com/ladymamawang/N_Commit_Tool/issues/4] 
2. ``-p``打印 git commit 提交信息
3. ``-m  test message`` 自定义提交内容 git commit -m "bugfix: test message (closed #3)(wf -l)"
4. ``-w`` 浏览器打开仓库Issue页面 gh issue list --web
5. ``-n`` 命令行交互式选择``ISSUES``，根据规则的最大版本号分支，创建新的开发分支，交互方式如图

<img width="976" alt="企业微信截图_f403a8f5-7b97-4c7b-8617-e5ebc190e81c" src="https://user-images.githubusercontent.com/40767043/196132518-30fb1452-ef66-4bae-bab2-bec96c0aaa83.png">

6. ``-n -c``, 命令行交互式选择``ISSUES``, 并且 `fetch` `origin` 分支, 交互式选择新增 dev 分支对应的 ``base branch``
<img width="858" alt="image" src="https://user-images.githubusercontent.com/40767043/196680577-eb443f57-e375-446d-ae40-208b48fcdb3e.png">


#### Q&A
1. ``MAC OS``会报错开发者异常，只要在``访达``中打开一次就行
