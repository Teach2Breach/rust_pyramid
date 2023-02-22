# pyre
A simple program in Rust that leverages the python embeddable package for windows to evade EDR.

Basically the program will first download the (signed) python embeddable package for windows, then download and run a specified py file. In this case, I harcoded the python script to be run, to point to one of my other repos here: https://raw.githubusercontent.com/Teach2Breach/Python-Windows-Reverse-Shell/master/reverse_shell.py . It's just a really simple python reverse shell script.

The concept is based on https://github.com/naksyn/Pyramid . This is a really simple implementation, but it has been tested against very popular EDRs, without issue. Just change the line that points to my reverse_shell.py script to point to whatever python script you want to run, then compile binary with 'cargo build --release' and run just the binary on target.

Example:

![image](https://user-images.githubusercontent.com/105792760/220647607-ce4295db-34bf-4227-8b85-48598d6eb350.png)

You can either download, modify, and rehost my reverse shell script, or use your own. Really you can run any python script that doesn't require additional dependencies. 

The project is also built in a way to allow for generating a dll instead of an exe. In theory the dll could be loaded with other techniques, but I stopped testing this project before I made sure the dll loads with injection techniques. IIRC I wanted to be able to convert the dll to shellcode and load as shellcode, but I moved on. If someone does it, let me know :)
