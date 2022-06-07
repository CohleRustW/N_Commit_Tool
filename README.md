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


#### 姿势
1. local branch name regex: r".\*issue#?\d+.*"
2. gh 鉴权
3. issue 内的 bug 统一提交时替换为 bugfix

#### 问题
1. ``MAC OS``会报错开发者异常，只要在``访达``中打开一次就行
