# Usage

It's recommended to use quotation marks for field values, unless it involves numbers without any symbols, in order to allow for spaces. Examples shown below.

There is no limitation on what the username and payment field has to be. As long as both provide a legit means of contact (email, FA, DA, ect..) and payment (paypal, crypto, bank, ect..), respectfully, it does not matter.

## Requests & Commissions

### Requests
``artm --req --art <art> --cont <contact> --cust <customer> --desc <description>``

Example: ``artm --req --cust "Lupe Jacobson" --cont "Kasey.Goyette18" --art "virtual" --desc "Lorem ipsum [...]"``

This will produce ``virtual - lupe jacobson.amr`` with the following contents:
```json
{
  "id": "63d4749e-3814-4861-915e-8fb01df14381",
  "date": "2019-04-02T03:54:25.297501-04:00",
  "customer": "Lupe Jacobson",
  "contact": "Kasey.Goyette18",
  "art": "virtual",
  "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus vitae scelerisque lectus. Proin id felis."
}
```

### Commissions
``artm --comm --cont <contact> --cust <customer> --desc <description> --pmt <payment> --price <price>``

Example: ``artm comm --cust "Alberta Mann" --cont "Chanel_McKenzie7" --art "Tonga" --price "$43" --pmt "5458-2118-9194-8514" --desc "Lorem ipsum [...]"``

This will produce ``tonga - alberta mann.amc`` with the following contents:
```json
{
  "id": "55b63ab9-397c-481d-92d3-bf7978d40d69",
  "date": "2019-04-02T02:09:10.351457-04:00",
  "art": "Tonga",
  "customer": "Alberta Mann",
  "contact": "Chanel_McKenzie7",
  "price": "$43",
  "payment": "5458-2118-9194-8514",
  "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque."
}
```

## YCH

YCH shares the same options as commissions but ``--desc`` is removed in favor of ``--slot`` and ``--ref``. It has been extended into it's own subcommand because it has more in common with [eBay-style](https://en.wikipedia.org/wiki/English_auction) auctions and can extended, if needed.

``artm ych --art <art> --cont <contact> --cust <customer> --pmt <payment> --price <price> --ref <reference>``

Example: ``artm ych --cust "Bessie Hettinger" --cont "Jack.Torphy75" --art "Synthesize" --slot 4 --price "$25" --payment "31VLNZXfcpoA68wPRuWSdrmT3jv5k" --ref "https://www.furaffinity.net/view/20700210/"``

This will produce ``synthesize - 4 - bessie hettinger.amy`` with the following contents:
```json
{
  "id": "16b5cd5a-9372-414c-85fd-952c14dd9205",
  "date": "2019-04-02T02:08:15.831915500-04:00",
  "customer": "Bessie Hettinger",
  "reference": "https://www.furaffinity.net/view/20700210/",
  "art": "Synthesize",
  "slot": "4",
  "contact": "Jack.Torphy75",
  "price": "$25",
  "payment": "31VLNZXfcpoA68wPRuWSdrmT3jv5k"
}
```

### Raffle

TBA. See issue [#1](https://github.com/antonwilc0x/artm/issues/1).