# codeforces-extractor
Program for extracting codeforces data.

# Explanation
So since copying the testcases from the browser everytime makes us lose time, I wanted to get the data from the website and store it locally in order run each testcase from commandline quickly. I actually saw some people who are doing this, however, I didn't see any repository about it. So I made one for myself, and wanted to share it with everyone so anyone can use. 

# Usage & Configuration/Personalization
To get the data from the contest, you should pass the contest no. For example if you want to get them from this contest, https://codeforces.com/contest/1720/problems you need to pass 1720 to program:
```cmd
python main.py 1770
```

The program, as default, copies them to the current working directory's ```test/``` folder. So basically ```./test/``` folder. If you want to change it, you can change this line:
```python
main(number, './test/')
```
to
```python
main(number, 'path/to/the/directory/')
```

# TODO
I am using [neovim](https://github.com/neovim/neovim) as the IDE or PDE currently. So I want to write a plugin that will execute everything in commandline, and integrate with neovim's features very well.

# Contribution
If you liked this project, please leave a star. If you have any questions about the project, you can create an issue. If you have any idea to get the project better, you can do some stuff and push. I think it would be fun!
