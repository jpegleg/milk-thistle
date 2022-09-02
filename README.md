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
(base) 🦀; export XTUID=$(cat /dev/urandom | head -n12 | b2sum | cut -c1-24 )
(base) 🦀; echo -e $(target/release/milk-thistle TID-e037_$XTUID >> /var/tmp/milk-thistle.log; tail -n1 /var/tmp/milk-thistle.log) | sed 's/\"//g' | sed 's/,//g' | sed 's/ -----BEGIN PUBLIC/-----BEGIN PUBLIC/g' | sed 's/.*-----BEGIN EC PRIVATE KEY-----/-----BEGIN EC PRIVATE KEY-----/g' | tee /var/tmp/"$XTUID".pem
-----BEGIN EC PRIVATE KEY-----
Proc-Type: 4ENCRYPTED
DEK-Info: AES-128-CBC1609D41D3C66CE356C84A5CCC809E921

CYVISeH4KEpp+3omaciUiPtvmfITTzw/kWLTFXnMzSUFGAJ+mshsOOFzMiEqF6+/
2s4kG8hPollztj8jhxMnrb0+cgWXqCWqSqj2XNmVSLlbtSQARxxsRZSrcrsW1ICp
kuRb3SKsEZaeSPkLsV7Zacj1cl8YQcwMnyzhgUuUjl0uVKttlUEt813/BGLjSJRq
U6R+EAGCIH0AUQi5A9MBy/k/ZM9XM0kq28NOMtEfvIc=
-----END EC PRIVATE KEY-----
-----BEGIN PUBLIC KEY-----
MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE4gUblQOsXcA0uPA3U9ELZ/jxVCTmsJtz
W3fwVjBwZ7Kz5iNh9OpV2sw1yRlN+7QfdS9JJLYId6uPx75+Jvky7E8s4nTTowzW
vIeVoxjdR/1EzPG+0lZhQtpoZcBDBggL
-----END PUBLIC KEY-----

(base) 🦀; tail -n1 /var/tmp/milk-thistle.log
2022-09-02 06:52:25.902623189 UTC - Some("TID-e037_b3dfd6508df50fef7b02476d") - Thistle Field: Milk Thistle Microservice - secp384r1 generation >-> "-----BEGIN EC PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nDEK-Info: AES-128-CBC,1609D41D3C66CE356C84A5CCC809E921\n\nCYVISeH4KEpp+3omaciUiPtvmfITTzw/kWLTFXnMzSUFGAJ+mshsOOFzMiEqF6+/\n2s4kG8hPollztj8jhxMnrb0+cgWXqCWqSqj2XNmVSLlbtSQARxxsRZSrcrsW1ICp\nkuRb3SKsEZaeSPkLsV7Zacj1cl8YQcwMnyzhgUuUjl0uVKttlUEt813/BGLjSJRq\nU6R+EAGCIH0AUQi5A9MBy/k/ZM9XM0kq28NOMtEfvIc=\n-----END EC PRIVATE KEY-----\n", "-----BEGIN PUBLIC KEY-----\nMHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE4gUblQOsXcA0uPA3U9ELZ/jxVCTmsJtz\nW3fwVjBwZ7Kz5iNh9OpV2sw1yRlN+7QfdS9JJLYId6uPx75+Jvky7E8s4nTTowzW\nvIeVoxjdR/1EzPG+0lZhQtpoZcBDBggL\n-----END PUBLIC KEY-----\n"

(base) 🦀;
```
