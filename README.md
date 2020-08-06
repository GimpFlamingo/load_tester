# load_tester
A very basic easy to configure and use Rust based load tester for web applications.

## Build
### Debug
In order to build the debug version open up Command Prompt, `cd` to the root of the project and run `.\debug.bat`.
This will compile the program, copy the config.ron file to the debug directory and then run the program.

### Release
To compile the program into release mode, `cd` to the root directory using Command Prompt as stated before, and run
`.\release.bat`. This will build the release version of the project and copy the config.ron file to the release directory.

## Features
This is a simple load tester that is very basic and allows for easy customization. It allows you to enter the urls you 
would like to test, and then it will go through each url with the provided number of runs.
After each run it will shuffle all of the urls in the array randomly. Each request will also be timed. Once all the runs
have completed an average response time will be calculated.
This will all then be output to a file for viewing after the runs are completed.

After the initial compile, if you wish to edit the urls or the number of runs you can simply edit the config.ron file
then run the program again.

## Config

### urls
This is where the user will hit the full urls that they want to load tester to hit.

Example:
```
urls: [
    "https://localhost:5001/api/Sheet/load?lineNum=11-L1&timestamp=2020-08-01-07-07-26",
    "https://localhost:5001/api/Sheet/load?lineNum=12-L3&timestamp=2020-08-01-07-07-26",
    "https://localhost:5001/api/Sheet/load?lineNum=11-L3&timestamp=2020-07-30-07-07-26",
    "https://localhost:5001/api/Sheet/load?lineNum=13-L8&timestamp=2020-08-03-04-07-26",
    "https://localhost:5001/api/Sheet/load?lineNum=12-L1&timestamp=2020-08-01-07-07-26",
    "https://localhost:5001/api/Sheet/load?lineNum=12-L2&timestamp=2020-08-03-07-07-26",
    "https://localhost:5001/api/Sheet/load?lineNum=11-L4&timestamp=2020-07-30-07-07-26",
    "https://localhost:5001/api/Sheet/load?lineNum=13-L6&timestamp=2020-08-02-04-07-26",
]
```
### runs
This dictates how many times all the urls will be called. 
```
runs: 5
```

### Full Config File
```ron
Config(
    // when typing in the urls make sure you also have the full routes and whatever parameters you are testing
    // each url will be it's own user. The more urls that are added, the more users that are created.
    urls: [
        "https://localhost:5001/api/Sheet/load?lineNum=11-L1&timestamp=2020-08-01-07-07-26",
        "https://localhost:5001/api/Sheet/load?lineNum=12-L3&timestamp=2020-08-01-07-07-26",
        "https://localhost:5001/api/Sheet/load?lineNum=11-L3&timestamp=2020-07-30-07-07-26",
        "https://localhost:5001/api/Sheet/load?lineNum=13-L1&timestamp=2020-08-03-04-07-26",
    ],
    runs: 5,
)
```
