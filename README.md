# shyfyr
Text encryption algorithm library that goes ФЫР
```
mod lib;
fn main(){	
	assert_eq!(lib::ser("мир"), "фыРФЫрФЫРФЫрФырФыРфыРФЫрФЫРФЫрФырФЫРфыРФЫрфырФЫрФЫРФЫР");
	assert_eq!(lib::de("фыРФЫрФЫРФЫрФырФыРфыРФЫрФЫРФЫрФырФЫРфыРФЫрфырФЫрФЫРФЫР"), "мир");
}
```
