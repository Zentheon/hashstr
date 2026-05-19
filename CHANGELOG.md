# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 0.3.0 - 2026-05-19

#### Bug Fixes
- 28b341c436b04373c5bf9186f23b46b896808fdb - remove non-trait `from_str` method, add `from_bytes` - @Zentheon

- e1b8b08f07b2f35236718fe5733fa287ff080d6e - casing conversions now return an FStr instead of String - @Zentheon

#### Continuous Integration
- bb1418fe3c6566af9944aeb04fa5d036a7604aa5 - **(release)** more release testing - @Zentheon
- f0943d98670ee670ae65abd3710a460a3de46d35 - **(release)** testing release workflow - @Zentheon
- 437035ac6a00ce9b992bafb89d735b3608930e0d - **(release)** expand release.yml with SBOM and attestation - @Zentheon
- dd04f212ba5316ad9d73c1238ef0dae38c548dcf - **(release)** rename `publish.yml` to `release.yml` and switch to trusted publishing - @Zentheon
#### Documentation
- 4944f9a97482d0c3919dc7be4ff05cc8b7fc8129 - **(sha3)** remove unnecessary `include_str!` - @Zentheon
#### Features
- b3bcfd68a34c798bf8b0f4d1e8bac827dade56e8 - no-std compatibility - @Zentheon

- 9781632381fa7bd9dbeafb1146f6df3eff4f3069 - add From impls between variants - @Zentheon

#### Miscellaneous Chores
- 54a96d6868350067a162bd869723d985993bf2f9 - **(cog)** improve Cargo.toml bumping mechanism - @Zentheon
- 7a6ad55caa2fbc39edfcee7f364d12b5402ab187 - **(deps)** update `sha3` to 0.12.0 - @Zentheon
- ada3b338587d9572466f353569ce8b4ac3e034ef - **(deps)** update `fstr` to 0.2.20 and update usage - @Zentheon
- fd220ef385f6261291168d377db258a7916b907e - **(deps)** cargo update - @Zentheon
- 9d647e179e7cdc0fac8c1f4db3bfc868674d15f5 - **(deps)** use published `hashstr-derive` - @Zentheon
- c8b5ba9d5cdd54f14b4690a29d29e3211ec75bf0 - **(nix)** devenv update - @Zentheon
- 4cae74ce233afd352371a22f60974ce402572a07 - remove unused code and allow linker warnings in `hashstr-derive` - @Zentheon

- 70b028d93c59c6add205640179fd8137dde86e7d - remove redundant `license-file` field in Cargo.toml - @Zentheon

- fb89eebdc0499d41c467c90dec2a9c4ba4a67d7b - criterion & project update stuff - @Zentheon

#### Refactoring
- 91eadcc87311858a70e831e095187d7fc509c89e - **(rename)** rename "slice" methods to "array" & `from_inner_unchecked` -> `from_array_unchecked` - @Zentheon
- 5afb467552d57a83994da479edaf28e0575e302a - rename all structs: `*Str` -> `*Hex` - @Zentheon

- 871f0d00c550262cf52b58baf58186bd2f3404d3 - switch from `const_hex` to `base16ct` - @Zentheon

- 662d11054041795fa48471478ccb05ecd5d58185 - rename some methods - @Zentheon

- 8bbc3552287268f7b397227b1a90a50b0c7fda42 - lay groundwork for base64 variants - @Zentheon


- - -

## 0.2.0 - 2026-05-04

#### Documentation
- 3ab22c90c757f695656cb5b1b3518f09ba0e627e - **(readme)** correct README.md relative path in `hash-str` root - @Zentheon
#### Miscellaneous Chores
- 53667d47ff6f164c5cb34c43765ae571601779a1 - **(deps)** use published version of `hash-str-derive` in `hash-str` - @Zentheon
#### Refactoring
- daec01a4fa54cca1b2c466138ad9f6a74ca465cd - **(rename)** apparently I did not make sure "hash-str" was not in - @Zentheon

- - -

## 0.1.0 - 2026-05-01

First release!!! ✨

Some notable initial features include:

* Support for the majority of algorithms implemented by RustCrypto
* (nearly) Completely unified API across all variants
* Separate options for both upper and lowercase encoding
* Fast and efficient implementations

#### Bug Fixes
- d15b389a1335562ee442dea37efacfb4e9e11cd3 - **(hash-strings-derive)** hash length is now properly calculated - @Zentheon
- 47f640a9b1336543449f9ee708d8387035c54285 - lingering old naming scheme - @Zentheon

- 320c010d4f25f052df6dc5d826c07dd8c50623d7 - corrected some error logic and added a LengthError test - @Zentheon

- a1f6f351ee29cac307c4c29c0468a09ee6dcc374 - correct `blake::Hash` conversion and length const - @Zentheon

#### Documentation
- 7b9b9690c82d6f927929d453ba819d8a1b4b0db0 - update README - @Zentheon

#### Features
- 0eff8403df3bd826797f33c31d9b14d8c0d52257 - **(hash-strings-derive)** improved error messages of string conversions - @Zentheon
- 77c378fffb4e656d0bd390593affd72310a3341a - add error field getters - @Zentheon

- f0d985ec817436bf243c7b27f99e0c124e4adfd8 - add uppercase variants - @Zentheon

- 41b774cb04476966544b6189dc2b83faa69bedbf - Much improved conversions - @Zentheon

- ba1681ec7bd1772ccbd7733a4fea5700da779e52 - add "serde" feature (default) - @Zentheon

- 018e7c79b75e13c52da75f54c5e17aa021771875 - add `ascon-hash256` and `bash-hash` - @Zentheon

- adb991501355a5d20e92a34e15cad2f488a30d64 - add `belt-hash`, `fsb` and `gost94` - @Zentheon

- e72704f32bc776da86c323582cb3fb09e0f67024 - add `groestl`, `jh` and `kupyna` - @Zentheon

- d0613ce2726db2a0fa1756b83158ff71f52da740 - add `streebog`, `shabal` and `skein` - @Zentheon

- 0bf19617bfbf1c7e6b62aa767f9f5529690e0b22 - add `sm3`, `tiger` and `whirlpool` - @Zentheon

- 1c1365f3e21b1061ce02bfdf8bd689bad3f38bc4 - add `blake2` and `blake3` - @Zentheon

- a542260d44af79efcde0a1b1d6a595ff086ab46e - improve `PartialEq` impls; use `constant_time_eq` - @Zentheon

- 46d68d44f84515a057c9279c67c78688580c7403 - implement `From<GenericArray>` - @Zentheon

- 55eca15257fbf6a9cf5ad920feaa1c1945db8573 - added `check_str` and `check` methods - @Zentheon

- 29d6bc7d7b9d70d45b710370b073dbb6b09517a3 - add `sha1-checked` - @Zentheon

- ae366b05142bac4eed26cc01d009d230134def17 - add `ripemd` - @Zentheon

- 8c8d798168ee9ef0220f6260be9e4ea01781feaa - added `md2`, `md4` and `md5` - @Zentheon

- d47ad5ebd423665c481718d34cb8c72572dce7e9 - add `sha3` - @Zentheon

- 10bb3b8b5e9de2657182e65f91b95f149c2cbf94 - initial commit - @Zentheon

#### Miscellaneous Chores
- e234d6d0b66cddb056fbe6db07b9352f1da69450 - release prep - @Zentheon

- 1a13f9611b4540a3d56cd01e1c28dab86c0ba187 - remove unused code - @Zentheon

- a1191a771c915f48f308f0560533eb29808f562d - update devenv - @Zentheon

#### Refactoring
- 9d429c0ba15410059d644021dea3be3077e89a83 - rename project to something more fitting - @Zentheon

- a6f14df8fd024bec86b53cda6cfe9f5feffab1bf - rename EncodingError to HexError - @Zentheon

- 5c77d30f148f12fa96f4af71bf0a3139b89242c1 - make wrapped FStr private - @Zentheon

- 8b27d47d3cdb59d2c3904e12f3494aea5056c0f9 - use own LengthError instead of `fstr::LengthError` - @Zentheon

- 3a9e66b9a8e52fbd3d1f9b5c726ce9a028a96592 - merge crates into `hash-strings` - @Zentheon

- 7f5e4f4dc313c70820b2b9cbd94c0a664e448169 - switch to wrapping `fstr::FStr` instead of `String` - @Zentheon

#### Tests
- eae0110da8c4bbe17e367104a979172876b6da3e - add HexError test - @Zentheon

- d31fc482b31244fe8b6a09e6e868f6c75996cf0b - add tests - @Zentheon


- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
