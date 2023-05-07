# File system entries

Leaf stores its file system entries in a table that has the following fields:

- `id`
- `parent`
- `package`
- `name`
- `hash`

Note that this system does not differentiate between files and symlinks. It is more concerned about representing the structure of a package and not its contents. If a symlink changes, the hash will change and leaf will notice the change.

### id

The id of the filesystem entry is unique and identifies the entry.

### parent

The parent id is a id to a filesystem entry that is a directory. If the parent id is `NULL`, this entry is seen as a root entry.

### package

Every file system entry is provided by a package, this number links the filesystem entry to the package.

### name

The name of the filesystem entry, must not be unique, because duplicate directories can exist.

### hash

The hash of a file is used for checking for user changes to the file. If the filesystem entry is a directory, the hash is `NULL`. If the filesystem entry is a symlink, the hash is computed of the path the symlink is pointing to.

# Example

The following file system tree is provided by the package with the id `1`:

```
/
|- etc
| |- leaf.conf
|- var
  |- cache
  | |- leaf.cache
```

The table entries would look as follows:

```
id  parent  pkgid   name        hash
------------------------------------
0   NULL    1       etc         NULL
1   0       1       leaf.conf   ".."
2   NULL    1       var         NULL
3   2       1       cache       NULL
4   3       1       leaf.cache  ".."
```
