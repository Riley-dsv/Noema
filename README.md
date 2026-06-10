# Noema

Noema is a note-taking application written in Rust.

It is inspired by knowledge management and Zettelkasten-oriented tools such as TriliumNext or Obsidian, while focusing on a native and lightweight approach.

## Current Features

At the moment, Noema is a minimal CLI-based note-taking application featuring:

- SQLite-based storage
- Terminal editor integration through `$EDITOR`
- Basic note CRUD operations
- Local persistent database
- UUID-based note identifiers

## Requirements

Noema expects the `EDITOR` environment variable to be set to a terminal editor such as:

- `vim`
- `nvim`
- `nano`
- `helix`

If no editor is configured, Noema falls back to `vi`.

## Database Location

By default, Noema stores its database using platform conventions:

### Linux / BSD

```
XDG_DATA_DIRS/noema/noema.db
```
or if `XDG_DATA_DIRS` does not exists

```
~/.local/share/noema/noema.db
```

### Windows

```
%APPDATA%\Noema\noema.db
```

### macOS

```
~/Library/Application Support/Noema/noema.db
```

You can override the default database location using:

```
noema note --db
```

## Usage

Initialize the database in your working directory:

```sh
noema init [path]
```

This creates the local SQLite database used to store your notes, if path is not specified, it fallsback to the default location.

### Create a note

```sh
noema note create --title "My note"
```

You can also provide initial content directly from the command line:

```
noema not create --title "My note" --content "# Initial content"
```

Once the editor is closed, the note content is saved to the database.

### List notes

```sh
noema note list
```

Displays the available notes and their IDs.

### Read a note

```sh
noema note read <ID>
```

Displays the content of a note.

### Update a note

```sh
noema note update <ID>
```

Opens the note in your configured editor.

### Search notes

```sh
notema note search <KEYWORD>
```

Search both titles and note contents for a keyword.

### Info about a note

```sh
noema note info <ID>
```

Display note metadata : 

* id
* title
* creation date
* update date

### Delete a note

```sh
noema note delete <ID>
```

Deletes the specified note.

## Help

```sh
noema --help
noema note --help
```

## Current State

Noema is currently in early alpha.

The project is still experimental and missing many planned features and improvements.

Contributions, issues, ideas, and patches are welcome.
