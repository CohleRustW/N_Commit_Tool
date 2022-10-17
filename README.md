### Node Man Commit Tool

```bash
 function ncommit () {
     /Users/......./ncommit $@
 }
 #追加以上函数到 .bashrc or .zsh_profile

```
#### 使用方式
1. ``brew install gh``
2. gh登录鉴权
3. 直接执行``ncommit`` -> git commit -m "bugfix: Test Issue (closed #4)(wf -l)" 参考: (issue#4)[https://github.com/ladymamawang/N_Commit_Tool/issues/4] 
4. ``-p`` 只打印命令，但是不执行
5. ``-m 1`` git commit -m "bugfix: 1 (closed #3)(wf -l)"
6. ``-w`` gh issue list --web
7. ``-n`` 命令行交互式选择``ISSUES``，根据规则的最大版本号rc分支，创建新的开发分支，交互方式如图

<img width="976" alt="企业微信截图_f403a8f5-7b97-4c7b-8617-e5ebc190e81c" src="https://user-images.githubusercontent.com/40767043/196132518-30fb1452-ef66-4bae-bab2-bec96c0aaa83.png">


#### 姿势
1. local branch name regex: r".\*issue#?\d+.*"
2. gh 鉴权
3. issue 内的 bug 统一提交时替换为 bugfix

#### 问题
1. ``MAC OS``会报错开发者异常，只要在``访达``中打开一次就行
