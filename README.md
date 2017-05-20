# hello everyone - welcome to project rust-game
in the early past 10 years i was very passionate about building my own game, so i tried many different game engines
and source codes i read them play with them but you know most of them were very complete at their own ways and they were very mature!
i can't deny that they all great! but i found that they are really big for some people like me.
i really like to build my game very simple and clean there should be a structure that you could read!
the pattern that you could understand, when they become larger and larger they become heavier to understand and
for those who want to just write a game they are great, but i think writing a game is an art and we should pay much attention on it,

so we are building an game engine from ground up here and i'm really focusing on it that anyone who came here to learn somthing from computer graphics, mathematics and other stuffs, also, i am really honored to start this project in github.

about the RUST!
i've tried many different languages for my project that non of them was the choice and i always failed with them, like c++ (really great choice but you have to be an expert with memory allocations), java (seriously it runs on every machine but it needs jre), c# (only windows i guess? and monogame? i tried xna in old times), js (were not building game for web browsers!), ...
so now that new language rust has come i found it interesting to build my game,
it is safe in memory and seg faults so? we are less concerned about memory leakage in game and we focuse more on building somthing great!


here are some modules i wrote first and they almost completed:


**math crate:**

**mat4** : matrices manupulation module! every game has a great one! everything you saw in the window are transform with matrices! built from ground up
**vec3** : simple vector module! not actually simple im trying to implement good functional
**quat** : quaternion for rotation and other stuffs...
  
core crate:
<br>
  this is the main application there is a simple window created by tomaka.github.io/glium crate, this is much like GLFW if you familiar with.
  to start this you should clone the repo and cd to core
  then run cargo run
  

```
git clone https://github.com/ary4n/rust-game/
cd core
cargo run
```

Screanshots: development in progress...

| project released | now |
| --- | --- |
|<img src="https://raw.githubusercontent.com/ary4n/rust-game/master/progress/screen1.png">|<img src="https://raw.githubusercontent.com/ary4n/rust-game/master/progress/screen2.png">|

