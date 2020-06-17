---
permalink: /docs/codebase/cli
---

# Teaclave Command Line Tool

The Teaclave command line tool (`teaclave_cli`) provides utilities to
interactive with the platform. The command line tool has several sub-commands:

- `encrypt`/`decrypt`: These two subcommands are to encrypt/decrypt data used on
  the platform. Supported algorithms include AES-GCM (128bit and 256 bit), and
  Teaclave File (128bit).
- `verify`: Verify the signatures of the enclave info (which contains `MRSIGNER`
  and `MRENCLAVE`) signed by auditors with their public keys. The enclave info
  is used for remote attestation, Please verify it before connecting the
  platform with the client SDK.
