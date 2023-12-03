
# Rustybox

The code is an implementation of various Linux command utilities in Rust. It defines functions for the following Linux commands : pwd , echo , cat , mkdir , mv , ln , rmdir , rm , ls , cp , touch , chmod.

In the main function , the command-line arguments are collected in the 'command_line_args' array and the command received is identified and placed in the 'first_command' variable and the corresponding function is called accordingly using the match statement . The command-line arguments are passed to the corresponding function using a slice that starts from index '2' (what comes after the name of the command). If any of the implemented functions propagate an error back to the caller(main function), the appropriate exit codes are returned or the 'Invalid command' message is printed in case of an invalid or unrecognized command.

Next,I will briefly describe the functionality of each command and how I implemented it:

-> pwd : pwd function prints the current working directory to the standard output or returns -1 in case of an error .

-> echo : echo function constructs a string by concatenating the command-line arguments into a buffer String and separate them by space . If the option is "-n", it does not add a newline at the end of the output. Otherwise, it adds a newline character to the output. The resulting string (buffer) is then printed .

-> cat : cat function reads the content of the files passed as arguments from the args_arr slice of strings and place the content in a buffer String ( concatenates their contents) ,then prints the concatenated content from the buffer to the standard output.

-> mkdir: mkdir function attempts to create each directory passed as argument from the args_arr slice of strings using the fs::create_dir function.In case of an error , it propagates the error to the main function .

-> mv : mv function takes two paths as arguments from the args_arr slice of strings, the source, and the destination. Depending on whether the destination path is a directory or a file, it either moves the source to the directory or renames the source to the destination file name.

-> ln : ln function first checks if a correct number of arguments were given.It then create a symbolic or a hard link depending on the option type (-s/--symbolic) and sets the boolean 'symbolic' accordingly.

-> rmdir : rmdir function takes each directory passed as argument from the args_arr slice of strings and creates a Path object , checks if the path exists and its pointing to a directory that is empty and then removes the empty directory. If a directory is not empty or the path does not exist, it exits the program with the appropiate error code.

-> rm : rm function takes the args_arr slice of strings as input, which represents the command-line arguments (files and directories given). It first checks for the options that were given ( "-r / -R / --recursive /-d / --dir ") .Then, it iterates through the input arguments, and for each argument, it constructs a Path object from it. If the Path points to a directory, it recursively removes the directory and its contents if the recursive option is set, or simply remove the directory if the directory-only option is set. If the Path points to a file, it removes the file. The 'deleted_entities' variable keeps track of the number of successfully deleted folders and files and if the number of deleted folders and files does not match the 'length' (number of files/directories that were passed as arguments), the program will return with the appropriate exit code (to handle the case where we have to delete files and dirs also but the option is not set).

-> ls : ls function checks for the type of option given and set the boolean 'show_hidden' or 'recursive' accordingly.Then , if the path given exists, it calls the 'list_directory_contents' . The list_directory_contents function takes three parameters: a reference to a Path object (path), a boolean flag 'show_hidden', and another boolean flag 'recursive'. It first checks whether path is a file or a directory and, if it's a file, simply prints the file path. If path is a directory, it iterates through its entries, filtering and printing the names of files and subdirectories based on the show_hidden flag. If the recursive flag is set, it also traverses subdirectories and calls itself recursively to list their contents.

-> cp :cp function first checks for the correct number of arguments and set the boolean 'recursive' to true if the option for recursive copy was given. It takes the source and destination arguments and creates a Path object for each. If the source does not point to an existing file/directory , it returns an error code. If the destination is not mentioned : it checks if the source is a file and copy the file with the name of the source . If the source is a directory , checks if the option recursive was set and creates the destination directory and then performs the copying by calling the 'copy_recursive' function. If the recursive option was not set , it will return an error. If the destination is mentioned : It checks if the recursive option is set and if it is ,calls the copy_recursive function ; Otherwise, it checks if the source path is a file and uses the copy function to copy the contents. If the source is a directory , the copying cannot be performed without the recursive option set so it returns an error code. The copy_recursive function copies the contents of a source directory or file specified by the source parameter to a destination directory specified by the destination parameter. If the source is a file, it copies the file to the destination using fs::copy. If the source is a directory, it first checks if the destination directory exists, and if not, it creates it using fs::create_dir. Then, it iterates over the entries (files and subdirectories) within the source directory, recursively calling itself on each entry to copy its contents to the corresponding location within the destination directory.

-> touch :touch function takes a slice of command-line arguments (args_arr) and checks for the option type. Depending on the options and file name provided, it either creates an empty file (if not already present and the -c option is not set), updates the access or modification time of the file, or performs a simple update by opening and immediately closing the file, thus updating access and modification times.

-> chmod : chmod function takes a slice of strings args_arr as input, representing command-line arguments that can be in numeric or symbolic mode followed by the name of the file. It first checks if the number of arguments is exactly 2 and handles invalid input accordingly. It then examines the first argument to determine whether it represents numeric or symbolic permissions. If it's numeric, it converts it to an octal mode and sets the file permissions accordingly using the fs::set_permissions function. If it's symbolic, it parses the symbolic permission string and sets the booleans (set_user,set_group,set_other) for (u/g/o) category as well as the (add_perm) for(+/-) to specify whether to add or remove permission. For read/write/execute permission , the 'tuple_perm' is updated accordingly. Then it applies the specified permission changes for users, groups, and others by setting the permission bits in the file's permission mode. Lastly,it uses the fs::set_permissions function to change the permissions of the file.

## Verify

Run the following commands to test your homework:

You will have to install NodeJS (it is installed in the codespace)

```bash
# Clone tests repository
git submodule update --init 

# Update tests repository to the lastest version
cd tests
git pull 
cd ..

# Install loadash
npm install lodash
```

Install rustybox

```bash
cargo install --path .
```

If the `rustybox` command can't be found, be sure to add the default cargo installation folder into the PATH environment variable

```bash
export PATH=/home/<your username here>/.cargo/bin:$PATH
```

Run tests

```bash
cd tests
# Run all tests 
./run_all.sh

# Run single test
./run_all.sh pwd/pwd.sh
```
