# milk-thistle
A fast and small program for secp384r1 key pair generation with rust openssl.

The private key is encrypted symmetrically with AES 128 CBC and a hard-coded passphrase in the milk-thistle binary.

Change the value that is set as CHANGEME in src/main.rs to the passphrase desired. Alternatively, read that passphrase from a config file or environment variable by updating src/main.rs accordingly.

The program takes a single argument which is simply logged in the output. This argument is designed to be used as a UUID or ID string to associate the usage.

The network design is to have milk-thistle within a remote (protected and authenticated) microservice.
The client caller then can pull a key pair without having the private passphrase stored locally or at least not directly exposing the private key and without using local generation. 
