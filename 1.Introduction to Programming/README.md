---
marp: true
---

# What is Programming ?

- Definition : the activity or job of writing computer programs
- It involves composing a sequence of instructions that computers can follow to perform tasks

---

# Types of Programming Languages ?

1. High Level Programming Languages
    - Easier to write (and read) for Humans
    - Allows representing real world concepts (ex: OOP)
    - Requires Compilation or Interpretation
    - Ex: Python, Java, Rust

2. Low Level Programming Languages: 
    - Hard to write for humans
    - Allow representation of basic computer instructions
    - Easy to transform into machine code for execution
    - Ex: Assembly Language for x86 arch or ARM etc


---

3. General Purpose Programming Language
    - Language for building software in a wide variety of application domains.
    - Ex: Rust, Python, Javascript etc

4. Domain-Specific Programming Language
    - Language designed for a specific application.
    - Ex: SQL

---

# The art of debugging

- It's a lucky day if your code compiles in one-go.
- Usually you would need to debug your code.
- To debug, you can use various techniques
    - Dry Run of the Program on paper, Change variable values after each step
    - Print out a sentence on the console from within the program
    - Use an IDE capable of debugging source code line by line
    - Comment out code and find the culprit
    - More creative techniques depending on current context

---

# How to think like a programmer ?

- Break down problems : A problem statement can usually be broken down into multiple smaller problems which are inter-connected. 
- Solve one-by-one : Solving multiple problems one by one is much easier than thinking about every problem at once.
- Remember basics: There are common patterns found in software such as functions, algorithms, loops, variables, logical operators, regular expressions etc. Being familiar with them helps.
- Clarity: Computers understand well defined steps without any ambiguity very well, hence a clear flow of the program needs to be understood beforehand. Computers can't yet process ambiguous statements.

---

- Pseudocode : A simple recipe is a good starting point before writing a complete program.
- No Distractions : Minimize distractions around you. A mind in flow/focused state can be much more productive. Context-Switching comes at a cost of lost context.
- Conversations : A conversation with a peer about the problem can really help solve the problem as it boosts clarity, and fascilitates a discussion about relevant topics.

---

# Using a Version Control System

- Often, source code goes through changes due to 
    - Bugfixes
    - New features
    - Security vulnerabilities
    - Third party API updates
    - etc
- Version control system helps in tracking changes to source code over time
- It also helps multiple people work on the same code base in parallel
- Example: Git, Apache SVN, Mercurial, Bitbucket

---

# Git Basics

- Install by following instructions here: `https://git-scm.com/downloads`
- Create a git repo
    ```
    mkdir <folder_name>
    cd <folder_name>
    git init
    ```
- Clone repo from external source
    ```
    git clone <repo url>
    ```
---

# Tracking changes using git

- Commit all file changes to git repo
    ```
    git add .
    git commit -m "summary of changes"
    ```
- Check status of git repo
    ```
    git status
    ```
- View past commits in the git repo
    ```
    git log
    ```

---

# Github

- A platform for developers to create, store, manage and share their code
- We can use git to interact with repos stored in Github