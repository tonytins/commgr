# Usage

It's recommended to use quotation marks for field values, unless it involves numbers without any symbols, in order to allow for spaces. Examples shown below.

There is no limitation on what the username and payment field has to be. As long as both provide a legit means of contact (email, FA, DA, ect..) and payment (paypal, crypto, bank, ect..), respectfully, it does not matter.

## Requests
``artm --art <art> --cont <contact> --name <name> --cust <customer> --desc <description>``

Example: ``artm --cust "Lupe Jacobson" --cont "Kasey.Goyette18" --name "virtual" --desc "Lorem ipsum [...]"``

```json
{
  "id": "36154b77e269bce9fca4c841a9ff7627dae4ad954fc8b9f8c3e28f063f2cfd6a",
  "date": "2019-04-06T15:46:46.395352700-04:00",
  "version": "0.1",
  "name": "virtual",
  "category": "Request",
  "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus vitae scelerisque lectus. Proin id felis.",
  "customer": {
    "name": "Lupe Jacobson",
    "contact": "Kasey.Goyette18"
  }
}
```

### Commissions
``artm --cont <contact> --name <name> --cust <customer> --desc <description> --pmt <payment> --price <price>``

Example: ``artm --cust "Alberta Mann" --cont "Chanel_McKenzie7" --name "Tonga" --price "$43" --pmt "5458-2118-9194-8514" --desc "Lorem ipsum [...]"``

```json
{
  "id": "6887f01a6c63c8130ea75a49d0ddff5b4cc5ad899ad127b396a09ba5559b63b1",
  "date": "2019-04-06T15:51:33.941149800-04:00",
  "version": "0.1",
  "name": "Tonga",
  "category": "Commission",
  "price": "$43",
  "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque.",
  "customer": {
    "name": "Alberta Mann",
    "contact": "Chanel_McKenzie7",
    "payment": "5458-2118-9194-8514"
  }
}
```

## YCH
``artm --name <name> --cont <contact> --slot <slot> --cust <customer> --pmt <payment> --price <price> --ref <reference>``

Example: ``artm --cust "Bessie Hettinger" --cont "Jack.Torphy75" --name "Synthesize" --slot 4 --price "$25" --payment "31VLNZXfcpoA68wPRuWSdrmT3jv5k" --ref "https://www.furaffinity.net/view/20700210/"``

```json
{
  "id": "fe8cf9771a9633e754cbafd0081e58bd3dcba286bdcc8012fb47c6eb0c3a6d33",
  "date": "2019-04-06T15:52:13.596792600-04:00",
  "version": "0.1",
  "name": "Synthesize",
  "category": "YCH",
  "slot": "4",
  "price": "$25",
  "reference": "https://www.furaffinity.net/view/20700210/",
  "customer": {
    "name": "Bessie Hettinger",
    "contact": "Jack.Torphy75",
    "payment": "31VLNZXfcpoA68wPRuWSdrmT3jv5k"
  }
}
```

### Raffle

TBA. See issue [#1](https://github.com/antonwilc0x/artm/issues/1).