# RAM relative position program

## indice
1. **approccio al problem**
2. **sull'implementazione**

## 1. approccio al problema
idea iniziale:
trovare su google una stampa di una moderna memoria RAM DDR4.
fatto questo, fare uno screenshot il piu' pulito possibile, vederne le dimensioni da cli, e trovare il rapporto altezza/lunghezza.

```
exiftool image_name.jpg | grep 'File Size'
```

come trovo il numero di "celle" nella ram?

**=>** la mia RAM e' di 8GB.
1 GB e' => 1024 MEGA che sono 1024 kilobyte
un gb e' 1024byte * 1024 * 1024 => (2^(10*3)) byte
la mia ram e' quindi questo per 8

2^33 byte
su C, un intero occupa <u>4 byte</u>. avremo quindi, in ottica di byte,
2^31 "celle per interi" e cio' basta e avanza come astrazione!

```
2^31 e' il numero di celle nella (mia!) RAM 
```

### come trovare riga e colonna
><u>abbiamo calcolato ora il numero di celle.</u>
come troviamo la posizione assoluta in questo _ipotetico_ rettangolo?


**divisone e modulo!**<br/>
_Nel dettaglio.._<br/>
Creiamo intanto un codice C che, allocando un intero
nell'heap, ci restituisca l'indirizzo **ASSOLUTO** della variabile considerata.

infatti,<br/>
|   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 00| 01| 02| 03| 04| 05| 06| 07| 08| 09| 10| 11| 12| 13| 14|
| 15| 16| 17| 18| 19| 20| 21| 22| 23| 24| 25| 26| 27| 28| 29|
| 30| 31| 32| 33| 34| 35| 36| 37| 38| 39| 40| 41| 42| 43| 44|


^^^^^^ Ipotizziamo una ram dove le celle sono disposte come segue