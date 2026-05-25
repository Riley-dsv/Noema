# Noema

A note taking software inspired by Zettelkasten like Obsidian, or Trilliumnext but built in Rust.

## Features

For now, it is a headless note taking software without any special capabilities.

## Use 

This software expect you to have an `EDITOR` env variable set to a texte editor like vim, nano, helix or other. By default it falls back to vi.

Once in your work folder you can call : 

```
noema init
```

To init the database file, every notes will be stored in it.

In your work folder you create a note using

```
noema note create --title <TITLE>
```

And after saving and quitting your note is stored in the database. 
To work on it after, you can just type.

```
noema note list
```
to retrieve the id of the note and 

```
noema note update <ID>
```

to write on it or : 

```
noema note read <ID>
```
To read it ( For now you can edit the note content, but it won't be saved )


And if you ever want to remove you note
```
noema note delete <ID>
```

All of those commands are available under : 

```
noema --help
noema note --help
```

## Current phase 

This is still in alpha, there is a lot of improvement to do, feel free to open a PR if you have idea of patch. 
