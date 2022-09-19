<center>

# Polkadot Blockchain Academy

## _Cambridge 2022_

</center>

This repository contains all student materials for the Cambridge 2022 Polkadot Blockchain Academy.

## Table of contents

- [Student Informational Materials](#student-informational-materials)
- [Shared Notes](#shared-notes)
- [Lesson slides with Reveal.js](#lesson-slides-with-revealjs)
- [Exercises, workshops and activities](#exercises-workshops-and-activities)
  - [Rust Jupyter notebooks with EvCxR](#rust-jupyter-notebooks-with-evcxr)
  - [Stand-alone Rust workshops and activities](#stand-alone-rust-workshops-and-activities)

## Student Informational Materials

An assortment of _key student information documents_ for the Academy exist in th [./materials/0-Student-Life/](./materials/0-Student-Life/) directory.

## Shared Notes

Th class has shared, collaborative, and curated lesson notes accessible here:

#### [PBA Cambridge 2022 - Shared Notes](https://docs.google.com/document/d/1l2Xuu7HhYhEVBT8kn_UlXN65leo6CTKEvSO58Df9dNQ/)

Please use _commenting mode_ to add your notes per class (this can be anonymous, use a private tab 😉). Feel free to add comments on anything here as well in a comment element (highlight text, `ctrl+alt+m`) too.

Some ideas of things to include in these notes:

- Useful links, related to the lesson
- References
- Ideas that come to mind
- Questions you had
- Feedback about the lesson
- Suggestions to improve

## Lesson slides with Reveal.js

**If this is your first time using `reveal.js`, we encourage you to explore [the official demo](https://revealjs.com/demo/#/2) to see what sort of things you can do with it!**
The Academy slides created using [`reveal-md`](https://github.com/webpro/reveal-md): a tool built with `reveal.js` to allow for [Markdown](https://commonmark.org/help/) only slides, with a few extra syntax items to make _your slides look and feel awesome_ with very little effort.

### Slides Quick Start

You will want an HTTP server to display the slides.
There are [many to choose from](https://gist.github.com/willurd/5720255), here we suggest using a JavaScript one managed by Yarn.

Have `nvm` and `yarn` already installed?
All you need to do is execute this from the top level directory of this repository:

```sh
# Ensure you have the right node
nvm i
# For yarn 3, you need to enable some node features
corepack enable
# Install Dependencies
yarn
# Run a slide server watching for file changes
yarn start
```

This should open a new tab with a listing of all slide decks to choose from.
Please start with the ["How to use Reveal slides"](./content-templates/slides/0-how-to-use-reveal-slides.md) guide to see what is possible with the slides features and some template slides.

<details>
<summary>If you are missing node or yarn, please install them as described below. (click to toggle)</summary>

### Node.js

For all linux and mac users We suggest to use [nvm](https://github.com/nvm-sh/nvm#installing-and-updating) to install and manage multiple node versions.
With `nvm` installed, from the academy top level dir:

```sh
nvm install
```

This will install (if needed) and set the correct version to use for this project set in the `.nvmrc` file here.

If you choose to not use `nvm`, you need [node](https://nodejs.org/en/) of version greater than `16.10`.
It is likely your [package manager](https://nodejs.org/en/download/package-manager/#debian-and-ubuntu-based-linux-distributions) has this version for you.

### Yarn

Please see the [official guide](https://yarnpkg.com/getting-started/install) to install for yarn 3.
Likely all you need to do is:

```sh
corepack enable
```

The only dependencies we need for this project can now all be installed with:

```sh
yarn
```

### Run to view slides

Running this command will open a new browser tab and _watch for file changes_ (update on every time you save a file in this repo):

```sh
yarn start
```

</details>

#### Viewing slides

Here are the basic ways to navigate through presenting your slides:

- Use `down/up` arrow keys to navigate _vertical_ slides.
- Use `left/right` arrow keys to navigate horizontal slides.
- Press `Esc` or `o` to see an `overview` view that your arrow keys can navigate. This allows you to click a slide to open it).
- Press `s` to open up speaker view - **this includes speaker notes that have extra information!**
  You will want to

## Exercises, workshops and activities

**The academy is focused on _practical application_ of Web3 concepts we cover, more than simply understanding.**
Each lecture may have a set of exercises, workshops and/or activities.
Here is how we define these:

- **Exercises**: these are short (5-10 minutes) exercises that are included as part of the slide deck and will be completing during the lecture.
- **Workshops**: these are step-by-step, longer (30 min to 3 hours) guided in-class material.
  These are instructor lead, and hand-held to get everyone to the same result.
- **Activities**: these are self-directed assignments for individuals and/or small groups that do not "hand-hold" like workshops.
  Student's completed work is expected to have some variety and a canonical solutions should be produced to review when students submit to compare to.

**Related materals will be made available in the [./materials/](./materials/) directory in this repository**

### Rust Jupyter notebooks with EvCxR

REPLs are a fantastic way to experiment with a language interactively.
[`evcxr_jupyter`](https://github.com/google/evcxr/tree/HEAD/evcxr_jupyter) uses [Jupyter Notebooks](https://jupyter.org/) for interactive documents with a built-in REPL.

**Please watch this [Jupyter 101 demo video](https://youtu.be/HW29067qVWk?t=248) to get to know the basics before proceeding**

#### _Quick start_

1. Install [`evcxr_jupyter`](https://github.com/google/evcxr/tree/HEAD/evcxr_jupyter#installation)
2. Open any `*.ipynb` in this repository with the tool of your choice:

- The easiest and likely to be suggested to students use notebooks is the [VSCode plugin](https://github.com/Microsoft/vscode-jupyter).
  Install by searching for this plugin (`@id:ms-toolsai.jupyter`) in the VSCode extensions menu.
  Once installed, as with `evcxr_jupyter` installed, you can select the Rust kernel and start interacting with Rust-based notebooks like the example.
- Best-in-class support is [Jupyter Lab](https://jupyterlab.readthedocs.io/en/stable/) over the vanilla notebooks tooling and VSCode.
  Installation [described here](https://jupyter.org/install).

> Note that sadly `rust-analyzer` [does not work with notebooks](https://github.com/rust-lang/rust-analyzer/issues/5141) at this time.

### Stand-alone Rust Workshops and Activities

For non-trivial Rust work, it's best to use a full IDE and cargo properly, over the REPL examples discussed above.
For these, we have dedicated crates that stand alone for each workshop or activity.

#### Local IDE

We suggest using [VSCodium](https://vscodium.com/) as it's most all-around featureful for academy work more than just Rust itself.

- We highly suggest [`sccache`](https://github.com/mozilla/sccache) that will enable faster builds for almost all academy students! If you want to use it globally, you need to add this with the right path to your `~/.cargo/config.toml` file:

  ```toml
  [build]
  rustc-wrapper = "<path to where>/.cargo/bin/sccache"
  ```

  Use `which sccache` to find the path.

- To get more power out of `sccache` and maybe overall faster linking, install and use the [`lld` linker](https://lld.llvm.org/) and while notebooks using EvCxR use this by default if detected, if you want to use it globally, see [this post](https://stackoverflow.com/questions/57812916/how-do-i-change-the-default-rustc-cargo-linker) on how to enable it. You need to add something like this to your `~/.cargo/config.toml` file:

  ```toml
  [target.x86_64-unknown-linux-gnu]
  rustflags = [
    "-C", "link-arg=-fuse-ld=lld",
  ]
  ```

#### Online IDE

There are some great (but limited) options for anyone lacking the ability to do things locally, or real-time collaboration is needed.

- [REPL.it](https://replit.com/languages/rust) - This online IDE includes a low-powered machine to compile Rust on, but provides _real-time collaboration_ where multiple users can follow-the-leader and edit simultaneously.
  It also features github integrations to save and share progress.
- [Substrate Playground](https://docs.substrate.io/playground/) - This is a Parity hosted very powerful build machine that anyone can access using a pre-compiled docker image of the base [substrate node template](https://github.com/substrate-developer-hub/substrate-node-template) to build on top of in an online VSCode-like IDE.
  The session is limited to 2 hours per user, before all data is cleared. There is no simple way to capture your work though, so this is only to be used in simple examples of substrate to iterate very quickly.
  _Note that it is unclear if 50+ students can access this at the same time, thus please let the TAs know if you plan to use this first!_
  _We should be able to give you longer living sessions for the Academy on request._
