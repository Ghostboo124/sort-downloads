# Sort Downloads

A tool to sort the windows or linux downloads folder.

## Usage

To use this, you install it with `cargo install sort-downloads` and then run `sort-downloads` or download it from the [GitHub Releases](https://github.com/Ghostboo124/sort-downloads/releases/latest) (though cargo is preferable) and run `./sort-downloads` on linux and powershell and `.\sort-downloads` on Windows command prompt and it will sort your downloads folder into 7 folders, documents, other, archives, executables, images, installers, and pdf. The folder a file goes in is based off of the following extentions

|      Folder | Extentions                                                            |
| ----------: | --------------------------------------------------------------------- |
|   documents | .doc, .docx, .docm, .ppt, .pptx, .pptm, .xlsx, .xlsm, .txt, .rtf, .md |
|    archives | .zip, .rar, .tar, .7z, .xz, .gz                                       |
| executables | .exe, .com, .bat, .cmd, .ps1, .sh, .bash, .zsh                        |
|      images | .svg, .png, .jpg, .jpeg, .webp, .gif                                  |
|  installers | .msi, .msix .AppImage                                                 |
|         pdf | .pdf                                                                  |
|       other | anything else                                                         |

## Building

To build this project clone it, then build it with `cargo build --release` or if you want debug information, then `cargo build`. If you want to build it then immediately run it, then run `cargo run --release` or if you want debug information in the built executable `cargo run`.

## Credits

This was also a solo project with some bugfixing done with the help of A.I.
