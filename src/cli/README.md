# Moss Cli


## Base usage / Syntax

The syntax for moss is as follows:

```bash
moss [OPTIONS] --comment <COMMENT> --user-id <USER_ID> [FILES]...
```

## Options

Moss takes two required parameters, some optional flags/options, and the list of files to submit.

The compulsory arguments are:

* **--comment**: The comment argument takes a string, as the title of the current submission. It can be shortened as  -c.
* **--user-id**: The user id option takes a string, is used to authenticate with Moss servers. It can be omitted as long as an environment variable named***MOSS_ID*** with the user id is it's value.

Other options are listed below.

| Name                                            | Option    | Short form | Type         | Default           |
| ------------------------------------------------- | :---------- | ------------ | -------------- | ------------------- |
| [Language](#language)                           | language  | l          | String       | C                 |
| [Directory mode](directory-mode)                | directory | d          | Boolean Flag | unset             |
| [Base file](#base-file)                         | base      | b          | file         | N/A               |
| [Ignore threshold](ignore-threshold)            | N/A       | m          | integer      | 10                |
| [Max matches displayed](#max-matches-displayed) | N/A       | n          | integer      | 250               |
| [Transformation](#transformation)               | t         | transform  | Regexp       | N/A               |
| [Experimental mode](experimental-mode)                | experimental | x          | Boolean Flag | unset             |
| [Server](#server)                               | server    | s          | url          | moss.stanford.edu |
| [Port](#port)                                   | port      | p          | integer      | 7690              |

### Language

The language parameter is used to specify the language of the code to be submitted. The following languages are supported:

[ c, cpp, java, ml, pascal, ada, lisp, scheme, haskell, fortran, ascii, vhdl, perl, matlab, python, mips, prolog, spice, vb, csharp, modula2, a8086, javascript, plsql ]

### Directory mode

The directory flag is used to tell Moss to treat all files in the same directory as an individual submission. Assuming we have files to submit in the folder structure below, the ***--directory** or ***-d*** flag tells moss to compare the contents of*person_1*to*person_2* and so on, rather than comparing the contents of

**project/person_1/file_1.txt** to **project/person_1/file_2.txt** to **project/person_2/file_1.txt** to **project/person_2/file_2.txt** and so on.

```
project  
│
└───person_1
│   │   file1.txt
│   │   file2.txt
│   │   ....
|
└───person_2
│   │   file1.txt
│   │   file2.txt
│   │   ....
|
...
|
└───person_n
│   │   file1.txt
│   │   file2.txt
│   │   ....
```

warning: Be careful when using in conjunction with the transform option. See [Transformation](#transformation) section for more information.

### Base file

A base file is a template file that serves as a template for a submission.

### Ignore threshold

The -m option allows the user to specify the upper limit for matches before they are ignored. This can be used to prevent matching trivial code. From the official Moss documentation,

> A passage of code that appears in many programs is probably legitimate sharing and not the result of plagiarism.  With -m N, any passage appearing in more than N programs is treated as if it appeared in a base file (i.e., it is never reported).
> Option -m can be used to control moss' sensitivity.  With -m 2, moss reports only passages that appear in exactly two programs.  If one expects many very similar solutions (e.g., the short first assignments typical of introductory programming courses) then using -m 3 or -m 4 is a good way to eliminate all but truly unusual matches between programs while still being able to detect 3-way or 4-way plagiarism.
> With -m 1000000 (or any very large number), moss reports all matches, no matter how often they appear. The -m setting is most useful for large assignments where one also a base file expected to hold all legitimately shared code.  The default for -m is 10.

### Max Matches Displayed

The -n option lets the user specify the number of matches to be shown in the final report.

### Transformation

The transformation option allows the user to apply a transformation to be applied to the absolute file path of the files. This can be useful to extract only certain information or to shorten file paths.

#### Background information

Moss displays file results based on their absolute file paths. This may be unwanted behavior for privacy reasons, or the inconvenience of displaying results with long file paths.

The user might only want to display certain information in the file path. For example, the user may want to display the results of the path `<pre>.../username_number/randomid_filename_timestamp<pre>` `<pre>.../username/filename<pre>` or better yet, `<pre>/username/filename<pre>`.

#### Path coercion on Windows

The moss client internally coerces all paths to *nix-style paths on Windows. This means that a file located at ***D:\path\to\\folder\file*** is transformed to **/path/to/folder/file**, with the drive information erased.

This is done as the Moss server only accepts *nix-style paths.

#### Path Transformation

The transformation option acts on the coerced file path on Windows and not on the original Windows-style file path.

The transform option creates a new virtual path by taking all capture groups in the first match of he coerced path and intersperses them with the *nix path separator (/).

The virtual path is then submitted to Moss

#### Example

Consider the folder structure below:

```
E:\very\long\folder\path
│
└───person_1
│   │   file1.txt
│   │   file2.txt
│   │   ....
|
└───person_2
│   │   file1.txt
│   │   file2.txt
│   │   ....
|
...
|
└───person_n
│   │   file1.txt
│   │   file2.txt
│   │   ....
```

Assuming all files were supplied to Moss without transforming, the will be submitted with the following coerced file paths:

1. /very/long/folder/path/person_1/file1.txt
2. /very/long/folder/path/person_1/file2.txt
3. /very/long/folder/path/person_2/file1.txt
4. /very/long/folder/path/person_2/file2.txt
5. /very/long/folder/path/person_3/file1.txt
6. /very/long/folder/path/person_3/file2.txt

For example, applying the transformation `<pre>`^.*/(.+)/(.+)$`</pre>` to the folder structure above would transform the paths to

1. /person_1/file1.txt
2. /person_1/file2.txt
3. /person_2/file1.txt
4. /person_2/file2.txt
5. /person_3/file1.txt
6. /person_3/file2.txt

### Experimental mode

The experimental mode option redirects submissions to the experimental Moss server. This may provide more features, but may result in a buggy experience.

### Server

The server option allows the user to override the server. It is meant to be used for testing purposes only.

### Port

The server option allows the user to override the port used in communicating to the server. It is meant to be used for testing purposes only.
