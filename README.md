# ![mme logo](logo-full.png)

[![Build Status](https://travis-ci.org/GoberInfinity/mme.svg?branch=master)](https://travis-ci.org/GoberInfinity/mme)

mme is a search tool that helps you to print commands and their description using a custom file.

## Installation

The binary name for rememberme is mme.

[Archives of precompiled binaries for rememberme are available for macOS and Linux.](https://github.com/GoberInfinity/mme/releases)

## Usage

You have to create the enviroment variable **MME_CFS** and set its value with the path where the file is going to be.

```bash
export MME_CFS = "~/../yourCommandFile"
```

Inside **yourCommandFile** put all the commands you want to remember in the following structure:

```
NAME
veryComplexCommand -a
DESC
A very explanatory description

NAME
veryComplexCommand -b
DESC
A very explanatory description
```

Finally search any command by NAME or DESC:

```bash
mme veryComplexCommand
```
