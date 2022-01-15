# cDateFunctions-Source
Includes source code of DLL files used in [cDateFunctions repository](https://github.com/ben-izd/cDateFunctions).


## Windows Workflow

You'll need to have rust and c++ compiler + Mathematica to access the header + lib file. If you hadn't:
- Visit [Rust official page](https://www.rust-lang.org/) and follow steps to download rust.
- Visit [Visual Studio Download page](https://visualstudio.microsoft.com/downloads/) install a version (I'd used community version). After installing, open "Visual Studio Installer" > "Modify" > check "Desktop Developement with C++" and click "Modify" (it'll download necessary files).

Steps to build the project:

1. Build the rust project.

2. Create a C++ Project in visual studio.

    1. Include Mathematica headers folder (example: `"C:\Program Files\Wolfram Research\Mathematica\13.0\SystemFiles\IncludeFiles\C"`)
    
    2. Link "wstp64i4m.lib" from Mathematica installation folder (example: `"C:\Program Files\Wolfram Research\Mathematica\13.0\SystemFiles\Links\WSTP\DeveloperKit\Windows-x86-64\CompilerAdditions"`)
    
    3. Link "cDateFunctionsLibraryLink.dll.lib" from rust "target/release" folder. (this file will be created if you execute step 1)
    
    4. build the project.

After following these steps, you'll have 2 dll files:
- "cDateFunctionsLibraryLink.dll" from building rust project which include main functions
- "cDateFunctionsLibraryLinkInterface.dll" from building C++ project which include the interface to connect to Wolfram-Language
