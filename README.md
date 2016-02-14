# timecutrust

[![Build Status](https://travis-ci.org/bogdad/timecutrust.svg?branch=master)](https://travis-ci.org/bogdad/timecutrust)

quickly find a time position in a log file (like nginx access)
using binary search, and then output all the lines newer than it

# usage
```
./target/debug/timecutrust

Usage: timecutrust [options] 'beg-time' file

Options:
    -r, --regexp REGULAR_EXPRESSION
          like ^\[(\d{4})-(\d{2})-(\d{2})\s(\d{2}):(\d{2}):(\d{2})\]
          default - ^\[(\d{4})-(\d{2})-(\d{2})\s(\d{2}):(\d{2}):(\d{2})\]
    -n, --nginx         run on nginx access.log with pattern like
                        '[28/Dec/2015:06:26:08'
                    
    -h, --help          print this help menu

beg-time:
    Time of start in the format specified in -r

file:
   Logfile to parse

examples:
timecutrust '[2015-12-28 20:37:25]' ./sample_g2.log
```