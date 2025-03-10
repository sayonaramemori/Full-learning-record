# git configuration

## Configure your git
> Local config file is located in .git/config

> Global config file is located in /etc/gitconfig or other place  
```
git config --list

git config --global user.name [YOUNAME]
git config --global user.email [YOUREMAIL]
git config --global core.editor vim
git config --global merge.tool vimdiff  

git config --global alias.[your-instruction-name] [instruction-name] 
//example: git config --global alias.unstage 'reset HEAD' 
```

## Gain Help  
```
git help <verb>
//for example: git help config
```

## Fundamental Instructions
```
git clone [URL] [DIR]

git init   //Initialize the current directory as a repository  
git status  //Most used in work
git reflog  //
git log  // --pretty=PARA  multiple parameters is available, such as oneline, full. Using double TAB to get more information.
git add FILES  //*是一个多功能命令，可以用来跟着新文件，或者把已跟踪的文件放入暂存区，还能用于合并冲突时把有冲突的文件标记为已解决状态;运行了git add之后又做了修改的文件，需重新运行git add把最新版本暂存起来
git commit FILES -a -m [comment] //skip add stage, commit directly  
git reset --hard [hash-number]  //Reset the head pointer to the specify version
```
## Git ignore  
```
# An Example  
# All files with suffix .a
*.a 
# But with an exception lib.a
!lib.a
# Ignore the file in / directory, not including subdir/TODO
/TODO
# Ignore the all the files located in build/ 
build/
# Ignore the doc/notes.txt, not including doc/server/arch.txt
doc/*.txt
```

## Difference  
```
git diff //check the difference between modified files and staged files.
git diff --cached //check the difference between the staged files and commited files lastly.
```

## Modify  
```
git rm FILES //untrack and remove the files  
git rm --cached FILES //untrack the files

/*
The Instruct below is equal to the three cmd
1. mv FILES NEW
2. git rm FILES
3. git add NEW
*/
git mv FILES NEW
```

## Amend Commit 
```
git commit --amend
/*example:
git commit -m "initial commit"
ait add forgotten_file
git commit --amend
*/
//Only one commit happens, the second commit amends the first commit.
```

## Unstage and untrack  
```
git reset HEAD FILES
git rm --cached FILE
```

## Discard modify  
```
git restore FILES
```

## Remote Repository  
```
git remote                              //Show remote repository name
git remote -v                           //Show name and corresponding url  
git remote show [NAME]                  //Show the remote repository detailed
git remote add NAME [URL]               //add a new remote repository 
git remote show [remote-name]           //Detailed information about the repository
git remote rename [old-name] [new-name] //rename the name 
git remote rm [remote-name]             //delete the local information of the remote repository
```

## Operation of the remote repository  
```
git fetch [remote-name] //Fetch the newest data without merging
git push [remote-name] [branch-name] //Merge to the remote repoistery, warrant is needed
git pull [remote-name] [branch-name] //Fetch the newest branch and try auto merge
```

## Branch Operation  
```
git branch //List the Branches
git branch -v

git branch [branch-name] //Create a new branch based on the current branch
git checkout [branch-name] //Switch branch
git checkout -b [branch-name] //Create a branch and switch to it
git merge [branch-name] //Merge the spcific branch to the current branch
git branch -d [branch-name] //Delete a branch

```

### Set upstream  
```shell
# Initial push and setting the upstream branch
git push -u origin feature-branch
# After running this command, future pushes and pulls can be done simply with
# Future push
git push

# Future pull
git pull
```

## Upload Big Files  
> Using [lfs](https://github.com/git-lfs/git-lfs)


## Add corresponding ssh-key for specific respository
```
git remote add [name] git@[alias]:sayonaramemori/study-roads.git

ssh-keygen -t ed25519 -C "your_email@example.com" -f ~/.ssh/id_ed25519_repo1
ssh-keygen -t ed25519 -C "your_email@example.com" -f ~/.ssh/id_ed25519_repo2

eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519_repo1
ssh-add ~/.ssh/id_ed25519_repo2

cd ~/.ssh
vim config

# Configuration for repository 1
Host github-repo1
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_ed25519_repo1

# Configuration for repository 2
Host github-repo2
    HostName github.com
    User git
    IdentityFile ~/.ssh/id_ed25519_repo2

# For repository 1
cd /path/to/repo1
git remote set-url origin git@github-repo1:your-username/repo1.git

# For repository 2
cd /path/to/repo2
git remote set-url origin git@github-repo2:your-username/repo2.git
```
