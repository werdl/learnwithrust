# Git and GitHub
Git is a CLI (we'll learn what that means in a moment) tool that is used to manage source code. It is a distributed version control system, which means that it allows multiple people to work on the same codebase at the same time. It also allows you to keep track of changes to your code over time, and to collaborate with others on your code.
## What is Git?
Git allows you and your friends/team/colleagues/classmates/random internet programmers to work on the same codebase at the same time. It also allows you to keep track of changes to your code over time, and to collaborate with others on your code.
## What is GitHub?
GitHub is a website that hosts Git repositories. It allows you to store your code in the cloud, and to collaborate with others on your code. It also provides a web interface for managing your code, and for viewing and discussing changes to your code.

## Installing Git
To install Git, visit the [Git website](https://git-scm.com/), and download the appropriate version for your operating system. Once you have downloaded the installer, run it, and follow the instructions to install Git on your system.

## Configuring Git
Once you have installed Git, you will need to configure it with your name and email address. This information will be used to identify you as the author of your code. To configure Git, open a terminal, and run the following commands:

```bash
git config --global user.name "Your Name"
git config --global user.email "Your Email"
```

Replace `Your Name` and `Your Email` with your actual name and email address.

## Creating a Git Repository
To create a new Git repository (code storage unit), navigate to the directory where you want to store your code, and run the following command:

```bash
git init
```

This will create a new Git repository in the current directory.

## Cloning a Git Repository
To clone (grab somebody else's code) an existing Git repository, run the following command:

```bash
git clone <repository-url>
```

Replace `<repository-url>` with the URL of the repository you want to clone.

## Adding Files to a Git Repository
To add files to a Git repository, run the following command:

```bash
git add <file>
```

Alternatively, to add all files in the current directory to the repository, run the following command:

```bash
git add .
```

This is because the `.` is a wildcard that matches all files in the current directory.

## Committing Changes to a Git repository
First, ensure that you have added the files you want to commit to the repository using the `git add` command.
To commit (save) changes to a Git repository, run the following command:

```bash
git commit -m "Your commit message"
```

Replace `Your commit message` with a brief description of the changes you are committing.

## Pushing Changes to a Remote Repository
To push (add) your changes to a remote repository (a repository hosted on GitHub, for example), run the following command:

```bash
git push
```

This will push your changes to the remote repository.

This will only work if you have previously configured a remote repository. To do this, run the following command:

```bash
git remote add origin <repository-url>
```

And that, in turn, will only work if you have previously created a repository on GitHub. To do this, visit the [GitHub website](https://github.com) and create a new repository (a handy guide can be found [here](https://docs.github.com/en/github/getting-started-with-github/create-a-repo)). Once you have created the repository, copy the URL, and use it in the `git remote add origin` command. 

One of these steps will ask you to login with your browser. If you are using a terminal that does not support this, you will need to use a different terminal or follow the instructions in the terminal to complete the login.

## Pulling Changes from a Remote Repository
To pull (grab) changes from a remote repository, run the following command:

```bash
git pull
```

## Other Git wizardry
There are many other things you can do with Git, such as branching, merging, rebasing, and more. We won't cover these in this guide, but you can find more information in the [official Git documentation](https://git-scm.com/doc).

Another useful resource is the [Pro Git book](https://git-scm.com/book/en/v2), which is available for free online.