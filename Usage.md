# Usage

**All arguments required**. It's recommended to use quotation marks for argument values, unless it involves numbers, in order to allow for spaces. Examples shown below.

## YCHs & Commissions

There is no limitation on what the username and payment field has to be. As long they both provide a legit means of contact (email, FA, DA, ect..) and payment (paypal, crypto, bank, ect..), respectfully, it does not matter.

### YCH

```
-c, --cust <customer>
-o, --order <order>
-p, --pay <payment>
-s, --slot <slot>
-u, --username <username>
```

Example: ``artm ych -c "William Moore" -u "dashawn @ twitter" -o "Alone tonight" -s 1 -p "kson7biigki@payment.com"``

This will produce ``alone tonight - william moore.amc`` with the following contents:
```json
{
  "id": "7f722e42-9e39-43c0-8d67-46293deb98fb",
  "customer": "William Moore",
  "order": "Alone tonight",
  "slot": "1",
  "username": "dashawn @ twitter",
  "payment": "kson7biigki@payment.com"
}
```

### Commissions

```
-c, --cust <customer>
-o, --order <order>
-p, --pay <payment>
-u, --username <username>
```
Example: ``artm com -c "William Moore" -u "dashawn @ twitter" -o "Alone tonight" -p "kson7biigki@payment.com"``
```json
{
  "id": "7f722e42-9e39-43c0-8d67-46293deb98fb",
  "customer": "William Moore",
  "order": "Alone tonight",
  "username": "dashawn @ twitter",
  "payment": "kson7biigki@payment.com"
}
```


## Requests

TBA