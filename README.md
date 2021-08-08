# ggez with resources.zip and windows path

The following command works fine. (non zip additional `resources/` dir)

```
$ cargo run --release
```

However, if I create `resources.zip` and `game.exe` and run `game.exe` as follows on Windows, it does not work. (`resources.zip`!)

```
$ cargo install --force cargo-make
$ cargo make
$ cd archive
$ ./game.exe
```

... And unzip `resources.zip` to `resources/`, then work fine. (`resources/` dir)

```
$ ./game.exe
```
