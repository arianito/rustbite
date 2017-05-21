# Welcome To Project rust-game
In the past 10 years I was very passionate about building my own game, so I tried many different game engines, I tried them, played with them, build tiny games with them, but you know they are very complex and very mature and well architected! I can't deny that they're all great!
but I really like to build my own game very simple and clean there should be a structure for engine that you could read! the game pattern that you could understand, when game engines become larger and larger they become harder to understand, I'm trying to build a game engine as simple as possible so everybody could understand and be able to efficiently scale and make it larger make it their own, while game engine it self keeping it's simplicity. for those who just want to make a game, todays game engine are just fine.

So, we are building a game engine here and I'm willing that everybody who come across, learns something, share some code, and become more knowledged about computer graphics, mathematics and other cool stuffs, also, I am really honored to start this project on github.

# Why RUST-LANG?
I tried many different languages for my project, and non of them was a good choice and i always failed because there was always and block in my way, 

| language | reason to reconsider | why rust?|
| --- | --- | --- |
| c++ | really great choice but you have to be an expert with memory allocations | rust can be both safe and fast |
| java | seriously it runs on every machine but it needs jre, it is not native! and works with GC! | rust have better memory management and efficiency |
| c#, dotnet | only windows i guess? and monogame? i tried xna in old times | rust provide cross platform compiling |
| js | were not building game for web browsers! | this is a poor review i know :( |

* with rust you are less concerned about memory and instead you can build something great.


# Project Crates

| crate | use case |
| --- | --- |
| math::mat4 | matrices manupulation module! every game has a great one! everything you saw in the window are transform with matrices!|
| math::vec3 | simple vector module! you know what vector is ?|
| math::quat | quaternion of rotation, used when you want to rotate something in screen |


| crate | use case |
| --- | --- |
| core::main | this is the main application, there is a simple window created by "tomaka.github.io/glium" crate, much like GLFW if you familiar with. |
  

# How To Start?
```
git clone https://github.com/ary4n/rust-game/
cd core
cargo run
```

Screanshots: development in progress...

| project released | now |
| --- | --- |
|<img src="https://raw.githubusercontent.com/ary4n/rust-game/master/progress/screen1.png">|<img src="https://raw.githubusercontent.com/ary4n/rust-game/master/progress/screen2.png">|

