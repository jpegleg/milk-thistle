# milk-thistle
A fast and small program for secp384r1 key pair generation with rust openssl.

The private key is encrypted symmetrically with AES 256 GCM and a hard-coded passphrase in the milk-thistle binary.

Change the value that is set as CHANGEME in src/main.rs to the passphrase desired. Alternatively, read that passphrase from a config file or environment variable by updating src/main.rs accordingly.

The program takes a single argument which is simply logged in the output. This argument is designed to be used as a UUID or ID string to associate the usage.

The network design is to have milk-thistle within a remote (protected and authenticated) microservice.
The client caller then can pull a key pair without having the private passphrase stored locally or at least not directly exposing the private key and without using local generation. 

In order to create the microservice, see prototype examples https://github.com/jpegleg/osprey_validator which can wrap this in web cgi etc. The milk-thistle binary would be included in a container image and executed via web request, likely called by a wrapper or gateway that passes the UUID and parses the output to return just the needed key data to the caller.

#### Using as a stand-alone CLI tool

Here is an example command line inflate to PEM formating from within the build git directory after `cargo build --release`:

```
(base) 🦀; echo -e $(target/release/milk-thistle TID-45727f7d8112bcf450572c0e9c9d87c3 >> /var/tmp/milk-thistle.log; tail -n1 /var/tmp/milk-thistle.log) | sed 's/\"//g' | sed 's/,//g' | sed 's/ -----BEGIN PUBLIC/-----BEGIN PUBLIC/g' | sed 's/.*-----BEGIN EC PRIVATE KEY-----/-----BEGIN EC PRIVATE KEY-----/g' > /var/tmp/45727f7d8112bcf450572c0e9c9d87c3.pem
(base) 🦀; echo -e $(target/release/milk-thistle TID-e0373662deeece51b3443c21cc3f4e2c >> /var/tmp/milk-thistle.log; tail -n1 /var/tmp/milk-thistle.log) | sed 's/\"//g' | sed 's/,//g' | sed 's/ -----BEGIN PUBLIC/-----BEGIN PUBLIC/g' | sed 's/.*-----BEGIN EC PRIVATE KEY-----/-----BEGIN EC PRIVATE KEY-----/g' | tee /var/tmp/e0373662deeece51b3443c21cc3f4e2c.pem
-----BEGIN EC PRIVATE KEY-----
Proc-Type: 4ENCRYPTED
DEK-Info: AES-128-CBC1D5CF6AF86F14CB3462E546D3F2AC22E

17oACSMbqpo3tRXAFd2QJv7JpL5oZYgUfmB/kQ8A+9TNGV1/++zZINwFfCmsHVyg
1e+1bwjLMrYut/RZ9w+Jq0rKNKha5kZ9bbrAB8DY8k6Mk07ezf/s3HhPcJAuu0CQ
HoqrgTB33LzKfFwKLSrA+KlfJPJebhPQ863UCOGqw/UvFHU8AsS/Wnk/w1O6TnuN
aTT5bx2OYOLkq6B0Rl4bGJYKmsyMc56RT5nkISs4p6w=
-----END EC PRIVATE KEY-----
-----BEGIN PUBLIC KEY-----
MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEOcozvmgOEpAvbfCGYmrRDDaayYnxxgii
EMT07BoYh/xHEyG9O49gq+vxAhsrPFNq42FyeDBBLSsX+Agi8oNc+g1Nkv6DiALy
80xhIN+88DsQL13Oth3j5kC6A4TyH4bZ
-----END PUBLIC KEY-----

(base) 🦀; cat /var/tmp/e0373662deeece51b3443c21cc3f4e2c.pem
-----BEGIN EC PRIVATE KEY-----
Proc-Type: 4ENCRYPTED
DEK-Info: AES-128-CBC1D5CF6AF86F14CB3462E546D3F2AC22E

17oACSMbqpo3tRXAFd2QJv7JpL5oZYgUfmB/kQ8A+9TNGV1/++zZINwFfCmsHVyg
1e+1bwjLMrYut/RZ9w+Jq0rKNKha5kZ9bbrAB8DY8k6Mk07ezf/s3HhPcJAuu0CQ
HoqrgTB33LzKfFwKLSrA+KlfJPJebhPQ863UCOGqw/UvFHU8AsS/Wnk/w1O6TnuN
aTT5bx2OYOLkq6B0Rl4bGJYKmsyMc56RT5nkISs4p6w=
-----END EC PRIVATE KEY-----
-----BEGIN PUBLIC KEY-----
MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEOcozvmgOEpAvbfCGYmrRDDaayYnxxgii
EMT07BoYh/xHEyG9O49gq+vxAhsrPFNq42FyeDBBLSsX+Agi8oNc+g1Nkv6DiALy
80xhIN+88DsQL13Oth3j5kC6A4TyH4bZ
-----END PUBLIC KEY-----

(base) 🦀; cat /var/tmp/milk-thistle.log                    
2022-09-02 06:16:05.919774868 UTC - Some("TID-45727f7d8112bcf450572c0e9c9d87c3") - Thistle Field: Milk Thistle Microservice - secp384r1 generation >-> "-----BEGIN EC PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nDEK-Info: AES-128-CBC,13B2AAF99DD3564CE84E6E826A5EC52E\n\n/C2SdT85FGnv+u/uMpFrBp7tWE01EK3xFNF/S9ojjKrcbFo57hw/niQYFBf59iYE\nLporDEeuJvO8Bxe1EBlG1lo5rkiwoUGnyKZzfcO3yYx9CsbG8KK2T5KXBXc3NTRw\nyykgtlwcrJ9X5Oxt4xNHFkshZtwbEgDrDeds7EQRH0lm9hI/lBTzDyoGIcHhAS04\nqgQIK5v+oR3kj80pSq+6SIa1G6ttTUQAXsb/wCgNdrI=\n-----END EC PRIVATE KEY-----\n", "-----BEGIN PUBLIC KEY-----\nMHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEmgkUw3N20JqVHGtx6Zaz+Tldz3aCue5L\n56zslMqNfdn9U1v31H4RIPb8kC1VCEgxiYU8r5LCc6kAnRmW4jsgBVWGHf/XDQqq\nd1TA0xFZDptpW3UR/VvH5iaJHTpwSBOY\n-----END PUBLIC KEY-----\n"
2022-09-02 06:16:15.392962522 UTC - Some("TID-e0373662deeece51b3443c21cc3f4e2c") - Thistle Field: Milk Thistle Microservice - secp384r1 generation >-> "-----BEGIN EC PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nDEK-Info: AES-128-CBC,1D5CF6AF86F14CB3462E546D3F2AC22E\n\n17oACSMbqpo3tRXAFd2QJv7JpL5oZYgUfmB/kQ8A+9TNGV1/++zZINwFfCmsHVyg\n1e+1bwjLMrYut/RZ9w+Jq0rKNKha5kZ9bbrAB8DY8k6Mk07ezf/s3HhPcJAuu0CQ\nHoqrgTB33LzKfFwKLSrA+KlfJPJebhPQ863UCOGqw/UvFHU8AsS/Wnk/w1O6TnuN\naTT5bx2OYOLkq6B0Rl4bGJYKmsyMc56RT5nkISs4p6w=\n-----END EC PRIVATE KEY-----\n", "-----BEGIN PUBLIC KEY-----\nMHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEOcozvmgOEpAvbfCGYmrRDDaayYnxxgii\nEMT07BoYh/xHEyG9O49gq+vxAhsrPFNq42FyeDBBLSsX+Agi8oNc+g1Nkv6DiALy\n80xhIN+88DsQL13Oth3j5kC6A4TyH4bZ\n-----END PUBLIC KEY-----\n"

(base) 🦀;
```
