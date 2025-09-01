# slexec
A cross-platform alternative to rundll32 on Windows.

## Support
- [x] Windows
- [x] macOS
- [x] Linux

## Installation
> [!NOTE]
> Binary releases are not currently provided.

Development releases can be installed with `cargo install --git https://github.com/Deltara3/slexec.git`.

## Usage
> [!WARNING]
> Entry points should follow the function signature of `void myfunc(const char **args)`.
> Anything else is undefined behavior, much like rundll32.

The CLI is extremely simple. For example, to execute `myfunc` in `mylib.dll` you can run:
```bash
slexec --module mylib.dll --function myfunc
```

Arguments can be provided to a function by passing a comma separated list like so:
```bash
slexec --module mylib.dll --function myfunc --pass myarg1,myarg2,myarg3
```

How these arguments are handled is up to the function.

## Motive
I was bored. I think it's a bit messy still and have to clean it up later.

## License
This project is licensed under the MIT License.
