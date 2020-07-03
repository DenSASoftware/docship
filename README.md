# docship - Ship your documentation with your program

This project is a small experiment on embedding documentation in the executable file of a rust binary. This project comes with a [mdbook](https://github.com/rust-lang/mdBook)-directory that's automatically built and embedded into the binary at compile-time. The only prerequisite is having mdbook installed on your system. After building the executable the documentation can be accessed by running `cargo run -- open-help`. The binary will bind a http-server to localhost on port 10101 and open your browser at this location.

The documentation is bound to the feature `docs`, which is enabled by default. Building without this feature enabled will exclude the http-server and the documentation from the binary and leaves a simple cli application.

### Why do this?

I came up with the idea when playing aroung with [jq](https://stedolan.github.io/jq/) and noticed that I had to visit the documentation more often than I wanted to. I could also read the manpage, but for some reason the website is easier to read. I also noticed that jq prides itself on being able to just be copied to another machine and work on there too. That gave me the idea of creating a program that contains its own documentation in form of a website and can deliver it too. It's just a POC and an exercise for me to get into rust build scripts.

### How it works

Behind the scenes the output of mdbook, some static files, are zipped and embedded as a giant byte array into the binary. At runtime the webserver answers requests by looking up filenames in the zip file. This way the entire documentation can be accessed as if the original directory lies on the filesystem, but it's compressed and packed into one single file without ever needing to be unpacked.

I used this [official build script example](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#code-generation) as a template to build on. The principle is the same, but I create a binary file instead of rust code.

### On binary size

The release build is 4.5MB big and consists of the following parts:

section          | size
---------------- | ----
symbols          | 1.7MB
server           | 1.5MB
docs (zip)       | 1.1MB
rest of the code | 0.2MB

Symbols can be removed easily with `strip <binaryname>`, but the rest is necessary for operation. Potential binary size improvements are a different mdbook theme and smaller crates. mdbook ships [fontawesome](https://fontawesome.io/) by default which comes with some big font files. Removing these files from the final zip resulted in a gain of 0.6MB. Suggestions are welcome.

