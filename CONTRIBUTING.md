# Contribution Guidelines
First of all, thanks for thinking about contributing to the GRDN
project. It's people like you that make this community awesome.

The following is a set of guidelines intended for every single
developer that wants to contribute to this proyect. By following them
you are communicating that you respect the time the developers
managing and contributing to this project have spent into making
it continuously better.

You can contribute in many different ways, from writing small
documentation and submitting bug reports, to requesting a new feature or
writing code that con be incorporated into the project. Everything is a
valid and very much appreciated contribution.

## Getting started
### Setting Up A Development Environment
The `anonymity-network` module is developed using the [Go programming
language](https://golang.org/). Be sure to install it in your system
before you start writing some code. You can follow the installation
tutorial on their web page if you don't know how to do it.

After installing it, remember to set your `$GOPATH` environment variable
so that it points to a convenient place to store all of your Go projects.
We recommend the following settings for an enjoyable development
experience:
  
  - Divide your `$GOPATH` into two. The first part will store every
    third-party dependency, while the second one will house your own
    development environment (in other words, your go workspace):
    ```bash
    $ export GOPATH=$HOME/golib # or other similar path
    $ export GOPATH=$GOPATH:<path-to-your-usual-devel-env>/go
    ```
  
  - Inside your go workspace, create a `src` folder, which will contain
    every single project you are working on. In it, create the following
    directory structure:
    ```
    src/
    └── github.com/
        └── GRDN-SVS/
    ```
    This should be done to follow the same structure that `go get`
    creates when getting a remote preject.

  - Clone your fork of the repo inside the `GRDN-SVS` directory.

Now you can actually start writting some code!