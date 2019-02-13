# rss-action
```
rss-action 0.1.0
nocent <nocent@protonmail.ch>
Perform an action for each unprocessed RSS item link

USAGE:
    rss-action [FLAGS] [OPTIONS] [--] <command>...

FLAGS:
    -h, --help              Prints help information
    -i, --ignore-failure    Do not abort when the specified command terminated with a non-zero exit code
    -V, --version           Prints version information

OPTIONS:
    -f, --feeds-path <feeds>              Set path to feeds list containing RSS URIs seperated by new lines [default:
                                          feeds.txt]
    -r, --replacement-string <replace>    Replace all occurences of the specified string in the arguments of <command>
                                          with each RSS item link
    -s, --state-path <state>              Set path to file containing already processed items [default: rss-action.dat]

ARGS:
    <command>...    The command to execute. The last argument will be the RSS item link unless overriden with an
                    option
```