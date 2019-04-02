# Usage

It's recommended to use quotation marks for field values, unless it involves numbers without any symbols, in order to allow for spaces. Examples shown below.

## Requests
Required fields:
```
--req
--client <client>
--art <art>
--desc <description>
--uname <username>
```

Example: ``artm --req --client "Lupe Jacobson" --uname "Kasey.Goyette18" --art "virtual" --desc "Lorem ipsum [...]"``

This will produce ``virtual - lupe jacobson.amr`` with the following contents:
```json
{
  "id": "63d4749e-3814-4861-915e-8fb01df14381",
  "date": "2019-04-02T03:54:25.297501-04:00",
  "client": "Lupe Jacobson",
  "username": "Kasey.Goyette18",
  "art": "virtual",
  "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus vitae scelerisque lectus. Proin id felis."
}
```

## YCHs & Commissions

There is no limitation on what the username and payment field has to be. As long as both provide a legit means of contact (email, FA, DA, ect..) and payment (paypal, crypto, bank, ect..), respectfully, it does not matter.

### Commissions
Required fields:
```
--comm
--client <client>
--desc <description>
--art <art>
--payment <payment>
--price <price>
--ref <reference>
--uname <username>
```
Example: ``artm --comm --client "Alberta Mann" --uname "Chanel_McKenzie7" --order "Tonga" --price "$43" --payment "5458-2118-9194-8514" --desc "Lorem ipsum [...]"``

This will produce ``tonga - alberta mann.amc`` with the following contents:
```json
{
  "id": "55b63ab9-397c-481d-92d3-bf7978d40d69",
  "date": "2019-04-02T02:09:10.351457-04:00",
  "art": "Tonga",
  "client": "Alberta Mann",
  "username": "Chanel_McKenzie7",
  "cost": "$43",
  "payment": "5458-2118-9194-8514",
  "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque."
}
```

### YCH
Required fields:
```
--ych
--client <client>
--art <art>
--payment <payment>
--price <price>
--ref <reference>
--slot <slot>
--uname <username>
```

Example: ``artm --ych --client "Bessie Hettinger" --uname "Jack.Torphy75" --art "Synthesize" --slot 4 --price "$25" --payment "31VLNZXfcpoA68wPRuWSdrmT3jv5k" --ref "https://www.furaffinity.net/view/20700210/"``

This will produce ``synthesize - 4 - bessie hettinger.amy`` with the following contents:
```json
{
  "id": "16b5cd5a-9372-414c-85fd-952c14dd9205",
  "date": "2019-04-02T02:08:15.831915500-04:00",
  "client": "Bessie Hettinger",
  "reference": "https://www.furaffinity.net/view/20700210/",
  "art": "Synthesize",
  "slot": "4",
  "username": "Jack.Torphy75",
  "cost": "$25",
  "payment": "31VLNZXfcpoA68wPRuWSdrmT3jv5k"
}
```

## Raffle

TBA