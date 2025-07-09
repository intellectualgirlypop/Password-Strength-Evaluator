Author: Trinity Wilson 
Email: intellectualgirlypop@gmail.com
Date started 6/17/25 - Date completed 

# Password-Strength-Evaluator
Utilizes Rust to analyze password strength by flagging common vulnerabilities and patterns.

## Project Goals 
⭐ Prompts user for password to validate 
⭐ Compares the entered password to flagged patterns and vulnerabilities
⭐ Outputs a password strength score and provides a summary with reasoning and tips if needed

## Core Components 
🐸 prompts user for password 
🐸 checks for basic features of a good password:
-length, use of upper and lower case <>
-use of numerical characters 
-use of special characters 
-ensuring no patterns or common passwords 
🐸 compares passwords to hardcoded features to produce a strength score 
🐸 outputs a score and tips to improve password(if needed)


## Things I Learned 
👾git is NOT to be played with omg i almost cried trying to figure out an ssh key issue 
    What happened ?
        While working on my GitHub repository, I encountered a `Permission denied` error when trying to push my code. Although my Git configuration was set with the correct username and email (`intellectualgirlypop`), GitHub was still identifying me as a different user (`TaiCodesSometimes`). This happened because I was using HTTPS to push, and Git was relying on cached login credentials from a previous session. To fix the issue, I opened Windows Credential Manager using `rundll32.exe keymgr.dll,KRShowKeyMgr` and removed all stored GitHub credentials. When I attempted to push again, Git prompted me to log in. I signed in with the correct GitHub account and used a Personal Access Token (PAT) in place of my password. After re-authenticating, the push was successful. Through this troubleshooting process, I learned the difference between Git’s config identity (which labels commits) and GitHub authentication (which uses stored credentials), how to switch between HTTPS and SSH remotes, and how to use tools like `git remote -v`, `ssh -T git@github.com`, and credential manager utilities to debug access problems.

