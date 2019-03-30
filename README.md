# Art Manager

A command line interface for storing art request, commission, and YCH information. The application is a work in progress but contribution is welcomed.

It's based on the [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) in that it provides back-end functions for front-ends to take advantage of. The benefit to this is that it's language- and platform-agnostic.

## Usage
Note: **all fields required**.

### YCH

```
-c, --cust <customer>
-o, --order <order>
-p, --pay <payment>
-s, --slot <slot>
-u, --username <username>
```
Example: ``artm ych -c "William Moore" -u "dashawn @ twitter" -o "Alone tonight" -s 1 -p kson7biigki@payment.com``

There is no limitation on what the username and payment field has to be. As long they both provide a legit means of contact (e.g. email, FA, DA, ect..) and payment (paypal, crypto, bank, ect..), respectfully. 

## To-do

- [ ] Write to file
    - [ ] YCH: ``[order]_[slot]_[customer].yart``
- [ ] Submission Types
    - [ ] YCH
    - [ ] Commission
    - [ ] Request
    - [ ] Self
- [ ] Status

## Supported systems

- Linux 2.16.8+
- macOS 10.7+
- Windows 7+
    - Windows 10 October 2018 Update or above recommended