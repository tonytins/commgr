# Usage

```txt
USAGE:
    cra order [OPTIONS] --buyer <buyer> --fee <fee> --payment <payment>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --buyer <buyer>                
    -c, --currency <currency>          
    -d, --description <description>    
    -f, --fee <fee>                    
    -p, --payment <payment>            
    -r, --reference <reference>        
    -s, --slot <slot>                  
    -y, --ych <ych>    
```

### Commissions

``cra order --b <buyer> -f <fee> -d <description> -p <payment>``

Example: ``cra order -b "Alberta Mann" -f 43 -p "5458-2118-9194-8514" -d "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque."``

```csv
Date,Client,Fee,Payment,Description
10/26/2020,Alberta Mann,$43.00,5458-2118-9194-8514,"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum ut pretium enim. Sed a neque."
```

## YCH

``cra order --b <buyer> -f <fee> -r <reference> -d <description> -p <payment> -y <ych> -s <slot>``

Example: ``cra order -b "Bessie Hettinger" -s 4 -f 25 -p "31VLNZXfcpoA68wPRuWSdrmT3jv5k" -r "https://www.furaffinity.net/view/20700210/" -y Synthesize``

```csv
Date,Client,Reference,Fee,Payment,YCH,Slot
10/26/2020,Bessie Hettinger,https://www.furaffinity.net/view/20700210/,$25.00,31VLNZXfcpoA68wPRuWSdrmT3jv5k,Synthesize,4
```

### Raffle

TBA. 

See issue [#1](https://github.com/tonytins/artm/issues/1).