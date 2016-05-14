# puts!

Often times you want to print something in rust and have to type not 1, not 2, not 3, but 9 consecutive punctuation characters to do so! 

That's insane. So I made puts!

Instead of:
```
println!("{:?} {:?}", object1, object2);
```

it's now

```
puts!(object1, object2)
```

just make sure to add 
```
#[macro_use(puts)]
extern crate puts;
```
in your project.
