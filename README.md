# Noema

Noema is a note-taking application written in Rust.

It is inspired by knowledge management and Zettelkasten-oriented tools such as TriliumNext or Obsidian, while focusing on a native and lightweight approach.

## Current Features

At the moment, Noema is a minimal CLI-based note-taking application featuring:

- SQLite-based storage
- Terminal editor integration through `$EDITOR`
- Basic note CRUD operations
- Local persistent database

## Requirements

Noema expects the `EDITOR` environment variable to be set to a terminal editor such as:

- `vim`
- `nvim`
- `nano`
- `helix`

If no editor is configured, Noema falls back to `vi`.

## Usage

Initialize the database in your working directory:

```sh
noema init
```

This creates the local SQLite database used to store your notes.

### Create a note

```sh
noema note create --title "My note"
```

Your editor will open automatically.  
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
