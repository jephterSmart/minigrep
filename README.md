# Minigrep

This is a CLI tool that allows you to search for text (needle) in a file (haystack).

This tools tries to include the functionality of a basic grep tool that comes with linux distribution operating system.

## Functionality Used

- Use of fs module to read string in files
- Use of cli arguments to specify search string/query and file path. This uses the std::env::args function.
- Use of environment variables to determine what kind of search is being carried upon on the content of our file.
