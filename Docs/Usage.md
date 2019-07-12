# Usage

There is no limitation on what the username and payment field has to be. As long as both provide a legit means of contact (email, FA, DA, ect..) and payment (paypal, crypto, bank, ect..), respectfully, it does not matter.

## Compatibility Notes

- Categories was removed in 0.1.1 due to the differences in how Rust and .NET (de)serialize enums.

## Requests (default)

``[dotnet] artm [req] -C <contact> -n <name> -c <customer> -d <description>``

Example: ``[dotnet] artm [req] -c "Lupe Jacobson" -C "Kasey.Goyette18" -n "virtual" -d "Lorem ipsum [...]"``

```json
{
  "Id": "66b7a46c-7d29-450e-ba13-7d5346171ce6",
  "Date": "2019-04-06T15:46:46.395352700-04:00",
  "Version": "0.1.1",
  "Name": "virtual",
  "Description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque.",
  "Custmer": {
    "Name": "Lupe Jacobson",
    "Contact": "Kasey.Goyette18"
  }
}
```

### Commissions

``[dotnet] artm com -C <contact> -n <name> -c <customer> -d <description> -P <payment> -p <price>``

Example: ``[dotnet] artm com -c "Alberta Mann" -C "Chanel_McKenzie7" -n "Tonga" -p 43 -P "5458-2118-9194-8514" -d "Lorem ipsum [...]"``

```json
{
  "Id": "a7432b08-8170-4fd2-ad05-bb1a9700261a",
  "Date": "2019-04-06T15:51:33.941149800-04:00",
  "Version": "0.1.1",
  "Name": "Tonga",
  "Price": 43,
  "Description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque.",
  "Custmer": {
    "Name": "Alberta Mann",
    "Contact": "Chanel_McKenzie7",
    "Payment": "5458-2118-9194-8514"
  }
}
```

## YCH

``[dotnet] artm ych -n <name> -C <contact> -s <slot> -t <ticket> -c <customer> -P <payment> -p <price> -r <reference>``

Example: ``[dotnet] artm ych -c "Bessie Hettinger" -C "Jack.Torphy75" -n "Synthesize" -s 4 -t 1 -p 25 -P "31VLNZXfcpoA68wPRuWSdrmT3jv5k" -r "https://www.furaffinity.net/view/20700210/"``

```json
{
 "Id": "7a66144c-e110-4660-99d0-31f3558de9be",
 "Date": "2019-04-06T15:52:13.596792600-04:00",
 "Version": "0.1.1",
  "Name": "Synthesize",
  "Ticket": 1,
  "Slot": 4,
  "Price": 25,
  "Custmer": {
    "Name": "Bessie Hettinger",
    "Contact": "Jack.Torphy75",
    "Payment": "31VLNZXfcpoA68wPRuWSdrmT3jv5k"
  }
}
```

### Raffle

TBA. See issue [#1](https://github.com/antonwilc0x/artm/issues/1).