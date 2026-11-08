# Things done by Chish for in the parser.

- Defined some data structures

Better yet, Maybe I will explain how data is structured.

All the data about one HTML file is stored in one big struct, called `DocItem`

- it stores name... i.e whatever is in Heading of the file after keywords like Struct, Function, Macro, etc.
- path, like the Module path... std::vec::Vec.. not the file_path
- kind.. well kind is the type of rust token the documentation is about, and I determine that using an Enum ItemKind, which has variants like Module, Struct, Enum, Function, Macro, etc.
- declaration... How the thing is declared... extracted from the code block right below Title
- description is where the real information of stuff is stored.. I initally planned to have code examples and description separate, but some parts of the doc used examples like you would in a book, so I just kept them together
- then there is a list of available methods, the metadata for each method is also defined in a struct called MethodItem
