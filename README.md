# About

Porydelete is a CLI tool which allows you to delete specific parts of the 3rd generation decomp projects such as Pokemon, Items, Maps, Tilesets, Events and more. This allows ROM-hack creators to clean up their ROM(s).

# State of development

I want to spend the time for more important and fun things. That's why this repo is ARCHIVED!

# Features

| Feature | Works? | Bugless?| When?
|---------|--------|--------|------|
|FRLG/RS Support|❌|❌|?
|List|❌|❌|?
|Clear Map-Attributes|✅|✅|0.1.0 
|Delete Maps|✅|✅|0.2.0
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
