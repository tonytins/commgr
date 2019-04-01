# Usage

**All fields required**. It's recommended to use quotation marks for field values, unless it involves numbers, in order to allow for spaces. Examples shown below.

## YCHs & Commissions

There is no limitation on what the username and payment field has to be. As long they both provide a legit means of contact (email, FA, DA, ect..) and payment (paypal, crypto, bank, ect..), respectfully, it does not matter. The date uses your local time.

### YCH

```
-n, --name <name>
-o, --order <order>
-p, --pay <payment>
-c  --cost <cost>
-s, --slot <slot>
-u, --username <username>
```

Example: ``artm --ych -n "Bessie Hettinger" -u "Jack.Torphy75" -o "Synthesize" -s 4 -c "$25" -p "31VLNZXfcpoA68wPRuWSdrmT3jv5k"``

This will produce ``synthesize - 4 - bessie hettinger`` with the following contents:
```json
{
  "id": "ca5e3d11-98ff-47e7-81bb-8d2dc0442304",
  "date": "2019-03-31T21:29:45.239190300-04:00",
  "customer": "Bessie Hettinger",
  "order": "Synthesize",
  "slot": "4",
  "cost": "$25",
  "username": "Jack.Torphy75",
  "payment": "31VLNZXfcpoA68wPRuWSdrmT3jv5k"
}
```

### Commissions

```
-n, --name <name>
-o, --order <order>
-c  --cost <cost>
-p, --pay <payment>
-u, --username <username>
```
Example: ``artm --comm -n "Alberta Mann" -u "Chanel_McKenzie7" -o "Tonga" -c "$43" -p "5458-2118-9194-8514"``

This will produce ``tonga - alberta mann.amc`` with the following contents:
```json
{
  "id": "4f87188f-8dcf-4150-a609-ebcdd57ff107",
  "date": "2019-03-31T21:30:42.610605600-04:00",
  "customer": "Alberta Mann",
  "order": "Tonga",
  "cost": "$45",
  "username": "Chanel_McKenzie7",
  "payment": "5458-2118-9194-8514"
}
```


## Requests

TBA