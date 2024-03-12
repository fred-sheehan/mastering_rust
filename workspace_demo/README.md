# Earlier it was getting my head around git...

## Recently it's been getting to grips with Rusts modules and crates...

Git wins hands down, much easier to reason about :)

The trouble with crate/module config, is the error messages are pretty cryptic, they tell you what is wrong, but unlike the compiler when its creating a binary, it doesn't tell you what to do to fix it.

I did have some earlier files in the 'learning_rust' repo, where I pulled in some other crates to build my ascii_hangman game, so I had to have a look there, and see how the .toml files were set out, to get my head around the problem.

Anyway, this folder/files do work now, so if you too are stuck, have a look at the .toml files in EVERY directory, and see what info they need so the compiler can find everything it needs to build your project from a file structure.

- N.B. I did have a non standard file structure, with no src/lib.rts but that proved the most problematic part of the puzzle, so I had to change it to the standard file structure, at least for the lib file, and then it all worked.

Namaste ;)
