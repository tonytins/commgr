# Usage

It's recommended to use quotation marks for field values, unless it involves numbers without any symbols, in order to allow for spaces. Examples shown below.

### Commissions

``artm order --c <client> -f <fee> -d <description> -p <payment>``

Example: ``artm order -c "Alberta Mann" -f 43 -p "5458-2118-9194-8514" -d "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque."``

```csv
Date,Client,Fee,Payment,Description
10/8/2020,Alberta Mann,43,5458-2118-9194-8514,"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque."
```

## YCH

``artm order --c <client> -f <fee> -r <reference> -d <description> -p <payment> -y <ych> -s <slot>``

Example: ``artm order -c "Bessie Hettinger" -s 4 -f 25 -p "31VLNZXfcpoA68wPRuWSdrmT3jv5k" -r "https://www.furaffinity.net/view/20700210/" -y Synthesize``

```csv
Date,Client,Reference,Fee,Payment,YCH,Slot
10/8/2020,Bessie Hettinger,https://www.furaffinity.net/view/20700210/,25,31VLNZXfcpoA68wPRuWSdrmT3jv5k,Synthesize,4
```

### Raffle

TBA. 

See issue [#1](https://github.com/tonytins/artm/issues/1).