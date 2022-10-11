# rgit

rgit is a wrapper over the git command that applies the [user] section from .gitconfig file located in the nearest parent directory to the current repository.
The action is performed before each call of the rgit command, if necessary.

## Installation
### zsh
Edit ~/.zshrc. At the end of file add:

```#rgit
git(){
  rgit $@
}
