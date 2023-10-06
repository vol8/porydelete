# About

Porydelete is a CLI tool which allows you to delete specific parts of the 3.gen decomp projects such as Pokemon, Items, Maps, Tilesets, Events and more. This is a project inspired by the `strip_events.py` script on my [pret-decomp-resources](https://github.com/Voluptua/pret-decomp-resources) repository. 

I am working on this project because:
  - I am not good at coding with Rust and I want to get better using this language
  - I always wanted to create a romhacking Tool (even in the binary hacking times)
  - I am also working on my own romhack and I want to save space by removing unused content in the final product such as unused Items or Tilesets


# State of development

Second Release available! 0.2.0! Now you can delete Maps created with porymap. Important is that other maps may use the map you want to delete, such as for connections. This can cause errors so you have to delete them manually.

# Features

| Feature | Works? | Fully added?| When?
|---------|--------|--------|------|
|List|❌|❌|Anytime
|Check|❌|❌|Anytime
|Delete Map-Attributes|✅|✅|0.1.0 
|Delete Maps|✅|❌|0.2.0
|Delete Map-Tilesets|❌| ❌|0.3.0
|Delete Scripts|❌|❌|0.4.0
|Delete Pokemon|❌|❌|0.5.0
|Delete Items|❌|❌|0.6.0


# Getting started

Follow these instructions to build this project: [INSTALL.md](https://github.com/Voluptua/Porydelete/blob/main/INSTALL.md).
Then place `porydelete` into the root of your project. \
Run `./porydelete --help` for more information.

## Map-Attributes and Filtering/Defiltering:

This is how you delete Map-Attributes:\
```./porydelete attr <attribute>```\
\
This is how to filter/defilter Maps:\
```./porydelete attr-fil <mapname>```\
```./porydelete attr-defil <mapname>```\
\
\
\
Available attributes to delete (only one per execution): 
  - `connections`
  - `object_events`
  - `warp_events`
  - `bg_events`
  - `coord_events`

Examples: \
  `./porydelete attr-fil MyNewTown`\
  `./porydelete attr connections`\
  `./porydelete attr-defil MyNewTown`

## Map Deletion:

This is how you delete Maps:\
```./porydelete map MyMap```\
Important is that you can only delete maps created by porymap!  \
\
Examples: \
  `./porydelete map MyNewTown`\
  `./porydelete map LittlerootTown`
