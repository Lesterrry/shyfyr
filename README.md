# shyfyr
[![Crate](https://img.shields.io/badge/crates.io-v0.1.1-orange)](https://crates.io/crates/shyfyr)

Text encryption algorithm library that goes ФЫР
```
mod lib;
fn main(){	
	assert_eq!(lib::ser("мир"), "фыРФЫрФЫРФЫрФырФыРфыРФЫрФЫРФЫрФырФЫРфыРФЫрфырФЫрФЫРФЫР");
	assert_eq!(lib::de("фыРФЫрФЫРФЫрФырФыРфыРФЫрФЫРФЫрФырФЫРфыРФЫрфырФЫрФЫРФЫР"), "мир");
}
```
