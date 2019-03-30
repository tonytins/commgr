# Art Manager

This is a art manager command line interface for storing request, commission, and YCH information. 

It's based on the [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) in that it provides back-end functions for front-ends to take advantage of. The benefit to this is that it's language- and platform-agnostic.

## Usage
Note: **all fields required**.

### YCH

```
-n, --notify <contact>
-c, --cust <customer>
-o, --order <order>
-p, --pay <payment>
-s, --slot <slot>
```
Example:
``artm ych -c William Moore -n dashawn -o Alone in the dark -s 1 -p kson7biigki@example.com``

## To-do

- [ ] Write to file (``[order]_[slot]_[customer].artm``)
- [ ] Submission Types
    - [ ] YCH
    - [ ] Commission
    - [ ] Request
    - [ ] Self

## Supported systems

- Linux 2.16.8+
- macOS 10.7+
- Windows 7+
    - Windows 10 October 2018 Update recommended